use anyhow::Result;
use clap::Args;

use axum::{routing::get, Router};
use axum::extract::{Json, Path, Query, State};
use tokio_postgres::Client;
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{Value, json};

use crate::db::build_db;

#[derive(Args)]
pub struct ServeCommand {
    #[arg(short, long)]
    port: u16,
}

pub struct ServerState {
    pub db: Client,
}

async fn modules(State(state): State<Arc<ServerState>>, Path((network, id)): Path<(String, String)>) -> Json<Value> {
    Json(json!({ "data": 42 }))
}

impl ServeCommand {
    pub async fn run(self) -> Result<()> {
        println!("Starting server on port: {}", self.port);
        let db = build_db().await?;
        let state = Arc::new(ServerState {
            db,
        });
        let app = Router::new()
            .route("/{network}/package/{id}", get(modules))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind("0.0.0.0:7000").await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
