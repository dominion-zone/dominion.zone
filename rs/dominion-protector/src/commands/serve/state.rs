use std::collections::HashMap;

use tokio::sync::Mutex;
use tokio_postgres::Client;
use anyhow::Result;

use crate::{ai::AI, db::build_db, sui_client::SuiClientWithNetwork};

pub struct ServerState {
    pub db: Mutex<Client>,
    pub sui_clients: HashMap<String, SuiClientWithNetwork>,
    pub ai: AI,
}

impl ServerState {
    pub async fn new() -> Result<Self> {
        let db = Mutex::new(build_db().await?);
        Ok(Self {
            db,
            sui_clients: HashMap::from_iter([
                ("mainnet".to_owned(), SuiClientWithNetwork::new("mainnet").await?),
                ("devnet".to_owned(), SuiClientWithNetwork::new("devnet").await?),
                ("testnet".to_owned(), SuiClientWithNetwork::new("testnet").await?),
            ]),
            ai: AI::new().await?,
        })
    }
}
