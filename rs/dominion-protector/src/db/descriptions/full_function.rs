use std::str::FromStr;

use anyhow::{Context, Result};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use sui_sdk::types::Identifier;
use tokio_postgres::{Client, Row, Transaction};

use super::{function::FunctionDescription, function_entity::FunctionEntityDescription};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullFunctionDescription {
    #[serde(flatten)]
    pub function: FunctionDescription,
    pub entities: Vec<FunctionEntityDescription>,
}

impl FullFunctionDescription {
    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        function_name: String,
        db: &mut Client,
    ) -> Result<Option<Self>> {
        let function = FunctionDescription::read_from_db(
            module_id.clone(),
            network.clone(),
            function_name.clone(),
            db,
        )
        .await?
        .ok_or_else(|| anyhow::anyhow!("Function not found"))?;
        let entities =
            FunctionEntityDescription::read_from_db(module_id, network, function_name, db).await?;
        Ok(Some(Self { function, entities }))
    }

    pub async fn read_all_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Vec<Self>> {
        let functions =
            FunctionDescription::read_all_from_db(module_id.clone(), network.clone(), db).await?;
        let mut result = vec![];
        for function in functions {
            let entities = FunctionEntityDescription::read_from_db(
                module_id.clone(),
                network.clone(),
                function.function_name.clone(),
                db,
            )
            .await?;
            result.push(Self { function, entities });
        }
        Ok(result)
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let function_saved = self.function.save_to_db(tx).await?;
        tx.execute("DELETE FROM function_entity_descriptions
        WHERE package_id = $1 AND network = $2 AND module = $3 AND function = $4", 
        &[
            &self.function.module.address().to_hex_literal(),
            &self.function.network,
            &self.function.module.name().as_str(),
            &self.function.function_name,
        ]).await?;
        for entity in &self.entities {
            entity.save_to_db(tx).await?;
        }
        Ok(function_saved)
    }
}
