use std::str::FromStr;

use anyhow::{Context, Result};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use sui_sdk::types::Identifier;
use tokio_postgres::{Client, Row, Transaction};

use super::{
    full_function::FullFunctionDescription, module::ModuleDescription, structure::StructDescription,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FullModuleDescription {
    #[serde(flatten)]
    pub module: ModuleDescription,
    pub structs: Vec<StructDescription>,
    pub functions: Vec<FullFunctionDescription>,
}

impl FullModuleDescription {
    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Option<Self>> {
        let module =
            ModuleDescription::read_from_db(module_id.clone(), network.clone(), db).await?;
        if let Some(module) = module {
            let structs =
                StructDescription::read_all_from_db(module_id.clone(), network.clone(), db).await?;
            let functions =
                FullFunctionDescription::read_all_from_db(module_id.clone(), network.clone(), db)
                    .await?;
            return Ok(Some(Self {
                module,
                structs,
                functions,
            }));
        } else {
            Ok(None)
        }
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<()> {
        println!("Saving module: {:?}", self.module);
        tx.execute(
            "DELETE FROM module_descriptions
        WHERE package_id = $1 AND network = $2 AND module = $3",
        &[
            &self.module.id.address().to_hex_literal(),
            &self.module.network,
            &self.module.id.name().as_str(),
        ]
        ).await?;
        self.module.save_to_db(tx).await?;
        println!("Saving new struct descriptions");
        for struct_ in &self.structs {
            struct_.save_to_db(tx).await?;
        }
        println!("Saving new function descriptions");
        for function in &self.functions {
            function.save_to_db(tx).await?;
        }
        Ok(())
    }
}
