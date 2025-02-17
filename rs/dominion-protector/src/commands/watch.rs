use anyhow::Result;
use clap::Args;

#[derive(Args)]
pub struct WatchCommand {}

impl WatchCommand {
    pub async fn run(self) -> Result<()> {
        println!("Watching for changes...");
        Ok(())
    }
}
