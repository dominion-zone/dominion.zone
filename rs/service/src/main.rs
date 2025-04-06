use std::sync::Arc;

use axum::Router;
use clap::Parser;
use anyhow::Result;
use state::ServerState;

pub mod state;

#[derive(Parser)]
#[command(name = "service")]
#[command(about = "Service")]
pub struct Cli {
    address: String,
}

impl Cli {
    pub async fn run(self) -> Result<(), anyhow::Error> {
        let state = Arc::new(ServerState::new().await?);
        let app = Router::new()
            // .route("/{network}/known_packages", get(known_packages))
            /*.route(
                "/{network}/module/{module_id}",
                get(module_description),
            )*/
            .with_state(state);
        let listener = tokio::net::TcpListener::bind(self.address).await?;
        axum::serve(listener, app).await?;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await
}
