use std::str::FromStr;

use anyhow::{Context, Result};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use sui_sdk::types::Identifier;
use tokio_postgres::{Client, Row, Transaction};

use super::security_level::SecurityLevel;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionDescription {
    pub module: ModuleId,
    pub network: String,
    pub function_name: String,
    pub description: String,
    pub security_level: SecurityLevel,
    pub warnings: Vec<String>,
}

impl FunctionDescription {
    pub async fn create_table_if_needed(client: &Client) -> Result<()> {
        client.batch_execute("
            CREATE TABLE IF NOT EXISTS function_descriptions (
                package_id      CHAR(66) NOT NULL,
                network         VARCHAR(10) NOT NULL,
                module          TEXT NOT NULL,
                function        TEXT NOT NULL,
                description     TEXT NOT NULL,
                security_level  security_level NOT NULL DEFAULT 'Unknown / Unassessed',
                warnings        TEXT[] NOT NULL DEFAULT '{}',
                PRIMARY KEY(package_id, network, module, function),
                FOREIGN KEY(package_id, network) REFERENCES objects(id, network) ON DELETE CASCADE,
                FOREIGN KEY(package_id, network, module) REFERENCES package_modules(package_id, network, name) ON DELETE CASCADE,
                FOREIGN KEY(package_id, network, module) REFERENCES module_descriptions(package_id, network, module) ON DELETE CASCADE
            );
        ").await?;
        Ok(())
    }

    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        function_name: String,
        db: &mut Client,
    ) -> Result<Option<Self>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
    FROM function_descriptions
    WHERE package_id = $1 AND network = $2 AND module = $3 AND function = $4",
                    FunctionDescription::COLUMNS.join(", ")
                ),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                    &function_name,
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(None);
        }
        return Ok(Some(FunctionDescription::try_from(&rows[0])?));
    }

    pub async fn read_all_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Vec<FunctionDescription>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
    FROM function_descriptions
    WHERE package_id = $1 AND network = $2 AND module = $3",
                    FunctionDescription::COLUMNS.join(", ")
                ),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                ],
            )
            .await?;
        return rows
            .into_iter()
            .map(|row| FunctionDescription::try_from(&row))
            .collect();
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let rows = tx.execute(
            &format!(
                "INSERT INTO function_descriptions ({})
    VALUES ($1, $2, $3, $4, $5, $6)
    ON CONFLICT (package_id, network, module, function) DO UPDATE
    SET description = $4, security_level = $5, warnings = $6",
                FunctionDescription::COLUMNS.join(", ")
            ),
            &[
                &self.module.address().to_hex_literal(),
                &self.network,
                &self.module.name().as_str(),
                &self.function_name,
                &self.description,
                &self.security_level,
                &self.warnings,
            ],
        ).await?;
        Ok(rows > 0)
    }
}

impl TryFrom<&Row> for FunctionDescription {
    type Error = anyhow::Error;

    fn try_from(value: &Row) -> Result<Self> {
        let columns = value.columns();
        let package_id: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "package_id")
                .context("package_id column not found")?,
        );
        let network: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "network")
                .context("network column not found")?,
        );
        let module: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "module")
                .context("module column not found")?,
        );
        let function: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "function")
                .context("function column not found")?,
        );
        let description: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "description")
                .context("description column not found")?,
        );
        let security_level: SecurityLevel = value.get(
            columns
                .iter()
                .position(|c| c.name() == "security_level")
                .context("security_level column not found")?,
        );
        let warnings: Vec<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "warnings")
                .context("warnings column not found")?,
        );

        Ok(Self {
            module: ModuleId::new(
                AccountAddress::from_hex_literal(&package_id)?,
                Identifier::from_str(&module)?,
            ),
            network,
            function_name: function,
            description,
            security_level,
            warnings,
        })
    }
}

impl FunctionDescription {
    const COLUMNS: &'static [&'static str] = &[
        "package_id",
        "network",
        "module",
        "function",
        "description",
        "security_level",
        "warnings",
    ];
}
