use anyhow::Result;
use clap::Parser;
use dominion_protector::Cli;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await
}
