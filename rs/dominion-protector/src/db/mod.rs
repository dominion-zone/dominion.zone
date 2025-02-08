use anyhow::Result;
use std::env;
use tokio_postgres::{Client, NoTls};

pub mod objects;
pub mod sources;
pub mod descriptions;

pub async fn build_db() -> Result<Client> {
    let password = env::var("DOMINION_PSWD")?;
    let (db, connection) =
        tokio_postgres::connect(&format!(
            "postgresql://dominion:{}@dominion.zone:5432/dominion_protector",
            password
        ), NoTls).await?;
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    Ok(db)
}
