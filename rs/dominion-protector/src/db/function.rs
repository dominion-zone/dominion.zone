use serde::{Deserialize, Serialize};
use sqlx::{query_as, query, Error, Executor, FromRow, Postgres};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type,
)]
#[sqlx(type_name = "visibility")]
pub enum Visibility {
    Private,
    Public,
    Friend,
    Package,
}

#[derive(Debug, FromRow)]
pub struct Function {
    pub package_id: String,
    pub network: String,
    pub module_name: String,
    pub function_name: String,
    pub visibility: Visibility,
    pub is_entry: bool,
    pub is_initializer: bool,
    pub type_argument_count: i32,
    pub parameter_count: i32,
    pub return_count: i32,
    pub source_code: Option<String>,
}

impl Function {
    pub async fn load<'a, E>(
        executor: E,
        package_id: &str,
        network: &str,
        module_name: &str,
        function_name: &str,
    ) -> Result<Option<Self>, Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(query_as!(
            Function,
            "SELECT 
                package_id,
                network,
                module_name,
                function_name,
                visibility as \"visibility: Visibility\",
                is_entry,
                is_initializer,
                type_argument_count,
                parameter_count,
                return_count,
                source_code
            FROM functions
            WHERE package_id = $1 AND network = $2 AND module_name = $3 AND function_name = $4",
            &package_id,
            &network,
            &module_name,
            &function_name
        )
        .fetch_optional(executor)
        .await?)
    }

    /// Сохраняет (вставляет или обновляет) одну строку
    pub async fn save<'a, E>(&self, executor: E) -> Result<(), Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        query!(
            "INSERT INTO functions (
                package_id, network, module_name, function_name, visibility, is_entry,
                is_initializer, type_argument_count, parameter_count, return_count, source_code
            ) VALUES (
                $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11
            ) ON CONFLICT (package_id, network, module_name, function_name)
            DO UPDATE SET
                visibility = EXCLUDED.visibility,
                is_entry = EXCLUDED.is_entry,
                is_initializer = EXCLUDED.is_initializer,
                type_argument_count = EXCLUDED.type_argument_count,
                parameter_count = EXCLUDED.parameter_count,
                return_count = EXCLUDED.return_count,
                source_code = EXCLUDED.source_code",
            &self.package_id,
            &self.network,
            &self.module_name,
            &self.function_name,
            self.visibility as _,
            self.is_entry,
            self.is_initializer,
            self.type_argument_count,
            self.parameter_count,
            self.return_count,
            self.source_code.as_ref()
        )
        .execute(executor)
        .await?;

        Ok(())
    }

    /// Загружает все строки для указанного package_id
    pub async fn load_all_by_package<'a, E>(
        executor: E,
        package_id: &str,
        network: &str,
    ) -> Result<Vec<Self>, Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(query_as!(
            Function,
            "SELECT
                package_id,
                network,
                module_name,
                function_name,
                visibility as \"visibility: Visibility\",
                is_entry,
                is_initializer,
                type_argument_count,
                parameter_count,
                return_count,
                source_code
            FROM functions WHERE package_id = $1 AND network = $2",
            &package_id,
            &network
        )
        .fetch_all(executor)
        .await?)
    }

    /// Загружает все строки для указанного package_id и module_name
    pub async fn load_all_by_module<'a, E>(
        executor: E,
        package_id: &str,
        network: &str,
        module_name: &str,
    ) -> Result<Vec<Self>, Error>
    where
        E: Executor<'a, Database = Postgres>,
    {
        Ok(query_as!(
            Function,
            "SELECT 
                package_id,
                network,
                module_name,
                function_name,
                visibility as \"visibility: Visibility\",
                is_entry,
                is_initializer,
                type_argument_count,
                parameter_count,
                return_count,
                source_code
            FROM functions WHERE package_id = $1 AND network = $2 AND module_name = $3",
            &package_id,
            &network,
            &module_name
        )
        .fetch_all(executor)
        .await?)
    }
}
