use anyhow::Result;
use tokio_postgres::Client;

pub mod full_function;
pub mod full_module;
pub mod function;
pub mod function_entity;
pub mod module;
pub mod security_level;
pub mod structure;

pub use full_function::*;
pub use full_module::*;
pub use function::*;
pub use function_entity::*;
pub use module::*;
pub use security_level::*;
pub use structure::*;

pub async fn create_description_tables_if_needed(client: &Client) -> Result<()> {
    /*
        CREATE TYPE security_level AS ENUM (
        'Critical Risk',
        'High Risk',
        'Medium Risk',
        'Low Risk',
        'Best Practices Compliant',
        'Unknown / Unassessed'
    );
    CREATE TYPE entity_kind AS ENUM ('parameter', 'created');
         */
    ModuleDescription::create_table_if_needed(client).await?;
    StructDescription::create_table_if_needed(client).await?;
    FunctionDescription::create_table_if_needed(client).await?;
    FunctionEntityDescription::create_table_if_needed(client).await?;
    Ok(())
}
