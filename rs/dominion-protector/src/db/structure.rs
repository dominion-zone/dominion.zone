use anyhow::Result;
use sqlx::{query_as_unchecked, query_unchecked, Executor, FromRow, Postgres};

#[derive(Debug, FromRow)]
pub struct Structure {
    pub package_id: String,
    pub network: String,
    pub module_name: String,
    pub datatype_name: String,
    pub origin: String,
    pub field_count: i32,
    pub type_argument_count: i32,
    pub source_code: Option<String>,
    pub has_key: bool,
    pub has_copy: bool,
    pub has_drop: bool,
    pub has_store: bool,
}

impl Structure {
    pub async fn load<'e, E>(
        executor: E,
        package_id: &str,
        network: &str,
        module_name: &str,
        datatype_name: &str,
    ) -> Result<Option<Self>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        Ok(query_as_unchecked!(
            Structure,
            "SELECT * FROM structures
         WHERE package_id = $1 AND network = $2 AND module_name = $3 AND datatype_name = $4",
            &package_id,
            &network,
            &module_name,
            &datatype_name
        )
        .fetch_optional(executor)
        .await?)
    }

    pub async fn save<'e, E>(&self, executor: E) -> Result<()>
    where
        E: Executor<'e, Database = Postgres>,
    {
        query_unchecked!(
            "INSERT INTO structures (
            package_id, network, module_name, datatype_name, origin, 
            field_count, type_argument_count, source_code, 
            has_key, has_copy, has_drop, has_store
        ) VALUES (
            $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12
        )",
            &self.package_id,
            &self.network,
            &self.module_name,
            &self.datatype_name,
            &self.origin,
            self.field_count,
            self.type_argument_count,
            &self.source_code,
            self.has_key,
            self.has_copy,
            self.has_drop,
            self.has_store
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    pub async fn load_all_by_module<'e, E>(
        executor: E,
        package_id: &str,
        network: &str,
        module_name: &str,
    ) -> Result<Vec<Self>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        Ok(query_as_unchecked!(
            Structure,
            "SELECT * FROM structures WHERE package_id = $1 AND network = $2 AND module_name = $3",
            &package_id,
            &network,
            &module_name
        )
        .fetch_all(executor)
        .await?)
    }

    pub async fn load_all_by_package<'e, E>(
        executor: E,
        package_id: &str,
        network: &str,
    ) -> Result<Vec<Self>>
    where
        E: Executor<'e, Database = Postgres>,
    {
        Ok(query_as_unchecked!(
            Structure,
            "SELECT * FROM structures
            WHERE package_id = $1 AND network = $2",
            &package_id,
            &network
        )
        .fetch_all(executor)
        .await?)
    }
}
