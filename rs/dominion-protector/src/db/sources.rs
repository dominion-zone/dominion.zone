use std::str::FromStr;

use anyhow::{Context, Result};
use sqlx::{query_as_unchecked, query_unchecked, Executor, FromRow, Postgres};
use sui_types::base_types::ObjectID;

#[derive(Debug, FromRow)]
pub struct ModuleSource {
    pub package_id: String,
    pub network: String,
    pub module_name: String,
    pub source: String,
    pub kind: String,
}

impl ModuleSource {
    pub async fn load<'e, E>(
        executor: E,
        package_id: &ObjectID,
        network: &str,
        module_name: &str,
    ) -> Result<Option<Self>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        Ok(query_as_unchecked!(
            ModuleSource,
            "SELECT * FROM module_sources
            WHERE package_id = $1 AND network = $2 AND module_name = $3",
            &package_id.to_string(),
            &network,
            &module_name
        )
        .fetch_optional(executor)
        .await?)
    }

    pub async fn save<'e, E>(&self, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Postgres>,
    {
        query_unchecked!(
            "INSERT INTO module_sources (
            package_id, network, module_name, source, kind
        ) VALUES (
            $1, $2, $3, $4, $5
        )",
            &self.package_id,
            &self.network,
            &self.module_name,
            &self.source,
            &self.kind
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn known_packages<'e, E>(executor: E, network: &str) -> Result<Vec<ObjectID>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        let rows = query_unchecked!(
            "SELECT package_id FROM module_sources
            WHERE network = $1
            GROUP BY package_id",
            network
        )
        .fetch_all(executor)
        .await?;
        Ok(rows
            .into_iter()
            .map(|r| {
                ObjectID::from_str(&r.package_id)
                    .context(format!("Failed to parse package_id {}", &r.package_id))
            })
            .collect::<Result<Vec<ObjectID>>>()?)
    }
}
