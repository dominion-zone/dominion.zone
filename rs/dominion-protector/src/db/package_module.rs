use anyhow::Result;
use sqlx::{query_as_unchecked, query_unchecked, Executor, Postgres};
use sui_sdk::types::base_types::ObjectID;

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct PackageModule {
    pub package_id: String,
    pub network: String,
    pub module_name: String,
    pub module_bytecode: Vec<u8>,
}

impl PackageModule {
    pub async fn load<'a, E>(
        executor: E,
        package_id: &ObjectID,
        network: &str,
        module_name: &str,
    ) -> Result<Option<Self>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(query_as_unchecked!(
            PackageModule,
            "SELECT package_id, network, module_name, module_bytecode 
             FROM package_modules 
             WHERE package_id = $1 AND network = $2 AND module_name = $3",
            &package_id.to_string(),
            &network,
            &module_name
        )
        .fetch_optional(executor)
        .await?)
    }

    pub async fn save<'a, E>(&self, executor: E) -> Result<()>
    where
        E: Executor<'a, Database = Postgres>,
    {
        query_unchecked!(
            "INSERT INTO package_modules (package_id, network, module_name, module_bytecode) 
             VALUES ($1, $2, $3, $4) 
             ON CONFLICT (package_id, network, module_name) 
             DO UPDATE SET module_bytecode = EXCLUDED.module_bytecode",
            &self.package_id,
            &self.network,
            &self.module_name,
            &self.module_bytecode
        )
        .execute(executor)
        .await?;
        Ok(())
    }

    pub async fn load_all_by_package<'a, E>(
        executor: E,
        package_id: &ObjectID,
        network: &str,
    ) -> Result<Vec<Self>>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(query_as_unchecked!(
            PackageModule,
            "SELECT package_id, network, module_name, module_bytecode 
             FROM package_modules 
             WHERE package_id = $1 AND network = $2",
            &package_id.to_string(),
            &network
        )
        .fetch_all(executor)
        .await?)
    }
}
