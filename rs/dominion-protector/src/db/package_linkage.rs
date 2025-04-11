use sqlx::{query_as, query, Executor, FromRow, Postgres};
use sui_sdk::types::base_types::ObjectID;

#[derive(Debug, FromRow)]
pub struct PackageLinkage {
    pub package_id: String,
    pub network: String,
    pub dependency_id: String,
    pub upgraded_id: String,
    pub upgraded_version: i64,
}

impl PackageLinkage {
    pub async fn load<'a, E>(
        executor: E,
        package_id: &ObjectID,
        network: &str,
        dependency_id: &str,
    ) -> Result<Option<Self>, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        query_as!(
            PackageLinkage,
            "SELECT * FROM package_linkage WHERE package_id = $1 AND network = $2 AND dependency_id = $3",
            package_id.to_string(),
            network,
            dependency_id
        )
        .fetch_optional(executor)
        .await
    }

    pub async fn save<'a, E>(&self, executor: E) -> Result<(), sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        query!(
            "INSERT INTO package_linkage (package_id, network, dependency_id, upgraded_id, upgraded_version)
             VALUES ($1, $2, $3, $4, $5)
             ON CONFLICT (package_id, network, dependency_id) DO UPDATE
             SET upgraded_id = EXCLUDED.upgraded_id, upgraded_version = EXCLUDED.upgraded_version",
            self.package_id,
            self.network,
            self.dependency_id,
            self.upgraded_id,
            self.upgraded_version
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn load_all_by_package<'a, E>(
        executor: E,
        package_id: &ObjectID,
        network: &str,
    ) -> Result<Vec<Self>, sqlx::Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        query_as!(
            PackageLinkage,
            "SELECT * FROM package_linkage WHERE package_id = $1 AND network = $2",
            package_id.to_string(),
            network
        )
        .fetch_all(executor)
        .await
    }
}
