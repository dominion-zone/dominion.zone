pub mod error;
pub mod known_packages;
pub mod state;

use clap::Args;

use axum::{routing::get, Router};
use axum::extract::{Json, Path, Query, State};
use state::ServerState;
use tokio_postgres::Client;
use std::collections::HashMap;
use std::sync::Arc;
use serde_json::{Value, json};
use axum::http::StatusCode;
use std::result::Result;

use crate::db::build_db;
use crate::commands::serve::known_packages::known_packages;

#[derive(Args)]
pub struct ServeCommand {
    address: String,
}


async fn package_description(State(state): State<Arc<ServerState>>, Path((network, id)): Path<(String, String)>) -> Json<Value> {
    Json(json!({ "data": 42 }))
}

impl ServeCommand {
    pub async fn run(self) -> Result<(), anyhow::Error> {
        let db = build_db().await?;
        let state = Arc::new(ServerState {
            db,
        });
        let app = Router::new()
            .route("/{network}/known_packages", get(known_packages))
            // .route("/{network}/package/{id}", get(package_description))
            .with_state(state);
        let listener = tokio::net::TcpListener::bind(self.address).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
