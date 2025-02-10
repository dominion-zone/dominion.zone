use anyhow::Result;
use std::env;
use tokio_postgres::{Client, NoTls};

pub mod descriptions;
pub mod objects;
pub mod sources;

pub async fn build_db() -> Result<Client> {
    let database_url = env::var("DATABASE_URL")?;
    let (db, connection) = tokio_postgres::connect(&database_url, NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(db)
}

pub async fn clear_db(client: &mut Client) -> Result<()> {
    client
        .batch_execute("drop table function_entity_descriptions;")
        .await?;
    client
        .batch_execute("drop table function_descriptions;")
        .await?;
    client
        .batch_execute("drop table struct_descriptions;")
        .await?;
    client
        .batch_execute("drop table module_descriptions;")
        .await?;
    client.batch_execute("drop table module_sources;").await?;
    client
        .batch_execute("drop table package_type_origins;")
        .await?;
    client.batch_execute("drop table package_modules;").await?;
    client.batch_execute("drop table objects;").await?;
    Ok(())
}
