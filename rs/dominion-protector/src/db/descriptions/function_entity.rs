use std::str::FromStr;

use anyhow::{Context, Result};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use sui_sdk::types::Identifier;
use tokio_postgres::{Client, Row, Transaction};


#[derive(Debug, Clone, Copy, ToSql, FromSql, Serialize, Deserialize)]
#[postgres(name = "entity_kind")]
pub enum EntityKind {
    #[postgres(name = "parameter")]
    Parameter,
    #[postgres(name = "created")]
    Created,
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionEntityDescription {
    pub module: ModuleId,
    pub network: String,
    pub function: String,
    pub description: String,
    pub kind: EntityKind,
    pub index: Option<i32>,
    pub name: String,
    pub proper_name: Option<String>,
    pub type_: String,
    pub returned: Option<String>,
    pub transferred: Option<String>,
    pub shared: Option<String>,
    pub frozen: Option<String>,
    pub wrapped: Option<String>,
    pub modified: Option<String>,
    pub dropped: Option<String>,
    pub warnings: Vec<String>,
}

impl TryFrom<&Row> for FunctionEntityDescription {
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
        let kind: EntityKind = value.get(
            columns
                .iter()
                .position(|c| c.name() == "kind")
                .context("kind column not found")?,
        );
        let index: Option<i32> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "index")
                .context("index column not found")?,
        );

        let description: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "description")
                .context("description column not found")?,
        );

        let name = value.get(
            columns
                .iter()
                .position(|c| c.name() == "name")
                .context("name column not found")?,
        );

        let proper_name = value.get(
            columns
                .iter()
                .position(|c| c.name() == "proper_name")
                .context("proper_name column not found")?,
        );

        let type_ = value.get(
            columns
                .iter()
                .position(|c| c.name() == "type")
                .context("type column not found")?,
        );

        let returned = value.get(
            columns
                .iter()
                .position(|c| c.name() == "returned")
                .context("returned column not found")?,
        );

        let transferred = value.get(
            columns
                .iter()
                .position(|c| c.name() == "transferred")
                .context("transferred column not found")?,
        );

        let shared = value.get(
            columns
                .iter()
                .position(|c| c.name() == "shared")
                .context("shared column not found")?,
        );

        let frozen = value.get(
            columns
                .iter()
                .position(|c| c.name() == "frozen")
                .context("frozen column not found")?,
        );

        let wrapped = value.get(
            columns
                .iter()
                .position(|c| c.name() == "wrapped")
                .context("wrapped column not found")?,
        );

        let modified = value.get(
            columns
                .iter()
                .position(|c| c.name() == "modified")
                .context("modified column not found")?,
        );

        let dropped = value.get(
            columns
                .iter()
                .position(|c| c.name() == "dropped")
                .context("dropped column not found")?,
        );

        let warnings = value.get(
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
            function,
            kind,
            index,
            name,
            proper_name,
            description,
            type_,
            returned,
            transferred,
            shared,
            frozen,
            wrapped,
            modified,
            dropped,
            warnings,
        })
    }
}

impl FunctionEntityDescription {
    const COLUMNS: &'static [&'static str] = &[
        "package_id",
        "network",
        "module",
        "function",
        "kind",
        "index",
        "name",
        "proper_name",
        "description",
        "type",
        "returned",
        "transferred",
        "shared",
        "frozen",
        "wrapped",
        "modified",
        "dropped",
        "warnings",
    ];

    pub async fn create_table_if_needed(client: &Client) -> Result<()> {
        client.batch_execute("
            CREATE TABLE IF NOT EXISTS function_entity_descriptions (
                package_id      CHAR(66) NOT NULL,
                network         VARCHAR(10) NOT NULL,
                module          TEXT NOT NULL,
                function        TEXT NOT NULL,
                kind            entity_kind NOT NULL,
                index           INT,
                name            TEXT NOT NULL,
                proper_name     TEXT,
                description     TEXT NOT NULL,
                type            TEXT NOT NULL,
                returned        TEXT,
                transferred     TEXT,
                shared          TEXT,
                frozen          TEXT,
                wrapped         TEXT,
                modified        TEXT,
                dropped         TEXT,
                warnings        TEXT[] NOT NULL DEFAULT ARRAY[]::TEXT[],
                FOREIGN KEY(package_id, network) REFERENCES objects(id, network) ON DELETE CASCADE,
                FOREIGN KEY(package_id, network, module) REFERENCES package_modules(package_id, network, name) ON DELETE CASCADE,
                FOREIGN KEY(package_id, network, module) REFERENCES module_descriptions(package_id, network, module) ON DELETE CASCADE,
                FOREIGN KEY(package_id, network, module, function) REFERENCES function_descriptions(package_id, network, module, function) ON DELETE CASCADE
            );
        ").await?;
        Ok(())
    }

    pub async fn read_from_db(
        module_id: ModuleId,
        network: String,
        function_name: String,
        db: &mut Client,
    ) -> Result<Vec<Self>> {
        let rows = db
            .query(
                &format!("SELECT {}
        FROM function_entity_descriptions WHERE package_id = $1 AND network = $2 AND module = $3 AND function = $4",
                FunctionEntityDescription::COLUMNS.join(", ")),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                    &function_name,
                ],
            )
            .await?;

        return rows
            .into_iter()
            .map(|row| FunctionEntityDescription::try_from(&row))
            .collect();
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let rows = tx.execute(
            &format!(
                "INSERT INTO function_entity_descriptions ({})
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17)",
                FunctionEntityDescription::COLUMNS.join(", ")
            ),
            &[
                &self.module.address().to_hex_literal(),
                &self.network,
                &self.module.name().as_str(),
                &self.function,
                &self.kind,
                &self.index,
                &self.name,
                &self.proper_name,
                &self.description,
                &self.type_,
                &self.returned,
                &self.transferred,
                &self.shared,
                &self.frozen,
                &self.wrapped,
                &self.modified,
                &self.dropped,
                &self.warnings,
            ],
        ).await?;
        Ok(rows > 0)
    }
}
