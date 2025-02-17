use std::collections::HashMap;

use anyhow::Result;

use crate::{ai::AI, db::Db, sui_client::SuiClientWithNetwork};

pub struct ServerState {
    pub db: Db,
    pub sui_clients: HashMap<String, SuiClientWithNetwork>,
    pub ai: AI,
}

impl ServerState {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            db: Db::new().await?,
            sui_clients: HashMap::from_iter([
                (
                    "mainnet".to_owned(),
                    SuiClientWithNetwork::new("mainnet").await?,
                ),
                (
                    "devnet".to_owned(),
                    SuiClientWithNetwork::new("devnet").await?,
                ),
                (
                    "testnet".to_owned(),
                    SuiClientWithNetwork::new("testnet").await?,
                ),
            ]),
            ai: AI::new().await?,
        })
    }
}
