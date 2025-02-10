use std::str::FromStr;

use anyhow::{Context, Result};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use postgres_types::{FromSql, ToSql};
use serde::{Deserialize, Serialize};
use sui_sdk::types::Identifier;
use tokio_postgres::{Client, Row, Transaction};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructDescription {
    pub module: ModuleId,
    pub network: String,
    pub struct_name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_owned: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_owned: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
    pub warnings: Vec<String>,
}

impl StructDescription {
    const COLUMNS: &'static [&'static str] = &[
        "package_id",
        "network",
        "module",
        "struct",
        "description",
        "address_owned",
        "object_owned",
        "wrapped",
        "shared",
        "immutable",
        "event",
        "warnings",
    ];

    pub async fn create_table_if_needed(client: &Client) -> Result<()> {
        client.batch_execute("
            CREATE TABLE IF NOT EXISTS struct_descriptions (
                package_id      CHAR(66) NOT NULL,
                network         VARCHAR(10) NOT NULL,
                module          TEXT NOT NULL,
                struct          TEXT NOT NULL,
                description     TEXT NOT NULL,
                address_owned   TEXT,
                object_owned    TEXT,
                wrapped         TEXT,
                shared          TEXT,
                immutable       TEXT,
                event           TEXT,
                warnings        TEXT[] NOT NULL DEFAULT '{}',
                PRIMARY KEY(package_id, network, module, struct),
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
        struct_name: String,
        db: &mut Client,
    ) -> Result<Option<StructDescription>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
                    FROM struct_descriptions
                    WHERE package_id = $1 AND network = $2 AND module = $3 AND struct = $4",
                    StructDescription::COLUMNS.join(", ")
                ),
                &[
                    &module_id.address().to_hex_literal(),
                    &network,
                    &module_id.name().as_str(),
                    &struct_name,
                ],
            )
            .await?;
        if rows.is_empty() {
            return Ok(None);
        }
        return Ok(Some(StructDescription::try_from(&rows[0])?));
    }

    pub async fn read_all_from_db(
        module_id: ModuleId,
        network: String,
        db: &mut Client,
    ) -> Result<Vec<StructDescription>> {
        let rows = db
            .query(
                &format!(
                    "SELECT {}
                    FROM struct_descriptions
                    WHERE package_id = $1 AND network = $2 AND module = $3",
                    StructDescription::COLUMNS.join(", ")
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
            .map(|row| StructDescription::try_from(&row))
            .collect();
    }

    pub async fn save_to_db(&self, tx: &mut Transaction<'_>) -> Result<bool> {
        let rows = tx.execute(
            &format!(
                "INSERT INTO struct_descriptions ({})
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
                 ON CONFLICT (package_id, network, module, struct) DO UPDATE
                 SET description = excluded.description,
                     address_owned = excluded.address_owned,
                     object_owned = excluded.object_owned,
                     wrapped = excluded.wrapped,
                     shared = excluded.shared,
                     immutable = excluded.immutable,
                     event = excluded.event,
                     warnings = excluded.warnings",
                StructDescription::COLUMNS.join(", ")
            ),
            &[
                &self.module.address().to_hex_literal(),
                &self.network,
                &self.module.name().as_str(),
                &self.struct_name,
                &self.description,
                &self.address_owned,
                &self.object_owned,
                &self.wrapped,
                &self.shared,
                &self.immutable,
                &self.event,
                &self.warnings,
            ],
        ).await?;
        Ok(rows > 0)
    }
}

impl TryFrom<&Row> for StructDescription {
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
        let struct_: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "struct")
                .context("struct column not found")?,
        );
        let description: String = value.get(
            columns
                .iter()
                .position(|c| c.name() == "description")
                .context("description column not found")?,
        );
        let address_owned: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "address_owned")
                .context("address_owned column not found")?,
        );
        let object_owned: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "object_owned")
                .context("object_owned column not found")?,
        );
        let wrapped: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "wrapped")
                .context("wrapped column not found")?,
        );
        let shared: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "shared")
                .context("shared column not found")?,
        );
        let immutable: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "immutable")
                .context("immutable column not found")?,
        );
        let event: Option<String> = value.get(
            columns
                .iter()
                .position(|c| c.name() == "event")
                .context("event column not found")?,
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
            struct_name: struct_,
            description,
            address_owned,
            object_owned,
            wrapped,
            shared,
            immutable,
            event,
            warnings,
        })
    }
}
