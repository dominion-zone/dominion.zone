use std::str::FromStr;

use anyhow::Result;
use move_core_types::language_storage::ModuleId;
use tokio_postgres::Client;

pub async fn create_sources_tables_if_needed(client: &Client) -> Result<()> {
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS module_sources (
            package_id      CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            name            TEXT NOT NULL,
            source          TEXT NOT NULL,
            kind            VARCHAR(20) NOT NULL,
            dependencies    TEXT[] NOT NULL,
            PRIMARY KEY(package_id, network, name),
            FOREIGN KEY(package_id, network) REFERENCES objects(id, network) ON DELETE CASCADE,
            FOREIGN KEY(package_id, network, name) REFERENCES package_modules(package_id, network, name) ON DELETE CASCADE
        );
    ").await?;
    Ok(())
}

#[derive(Debug, Clone)]
pub struct ModuleSource {
    pub id: ModuleId,
    pub network: String,
    pub source: String,
    pub kind: String,
    pub dependencies: Vec<ModuleId>,
}

pub async fn read_source_from_db(
    module_id: ModuleId,
    network: String,
    db: &mut Client,
) -> Result<Option<ModuleSource>> {
    let rows = db
        .query(
            "SELECT source, kind, dependencies
    FROM module_sources WHERE package_id = $1 AND name = $2 and network = $3",
            &[
                &module_id.address().to_hex_literal(),
                &module_id.name().as_str(),
                &network,
            ],
        )
        .await?;
    if rows.is_empty() {
        return Ok(None);
    }
    let dependencies: Vec<String> = rows[0].get(2);
    println!("Dependencies: {:?}", dependencies);
    return Ok(Some(ModuleSource {
        id: module_id,
        network,
        source: rows[0].get(0),
        kind: rows[0].get(1),
        dependencies: dependencies
            .into_iter()
            .map(|d| ModuleId::from_str(&d))
            .collect::<Result<Vec<_>>>()?,
    }));
}
