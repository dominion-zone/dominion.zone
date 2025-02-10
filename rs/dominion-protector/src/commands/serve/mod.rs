pub mod error;
pub mod known_packages;
pub mod module_description;
pub mod package_description;
pub mod state;

use clap::Args;

use axum::extract::{Json, Path, Query, State};
use axum::http::StatusCode;
use axum::{routing::get, Router};
use serde_json::{json, Value};
use state::ServerState;
use std::collections::HashMap;
use std::result::Result;
use std::sync::Arc;
use tokio_postgres::Client;

use crate::commands::serve::known_packages::known_packages;
use crate::commands::serve::module_description::module_description;
use crate::db::build_db;

#[derive(Args)]
pub struct ServeCommand {
    address: String,
}

impl ServeCommand {
    pub async fn run(self) -> Result<(), anyhow::Error> {
        let state = Arc::new(ServerState::new().await?);
        let app = Router::new()
            .route("/{network}/known_packages", get(known_packages))
            .route(
                "/{network}/module/{module_id}",
                get(module_description),
            )
            .with_state(state);
        let listener = tokio::net::TcpListener::bind(self.address).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}
