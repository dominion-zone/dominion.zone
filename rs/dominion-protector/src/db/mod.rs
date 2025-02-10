use anyhow::Result;
use std::env;
use tokio_postgres::{Client, NoTls};

pub mod descriptions;
pub mod objects;
pub mod sources;

pub async fn build_db() -> Result<Client> {
    let password = env::var("DOMINION_PSWD")?;
    let (db, connection) = tokio_postgres::connect(
        &format!(
            "postgresql://dominion:{}@dominion.zone:5432/dominion_protector",
            password
        ),
        NoTls,
    )
    .await?;
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
