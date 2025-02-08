use clap::Args;
use anyhow::Result;

#[derive(Args)]
pub struct ServeCommand {
    #[arg(short, long)]
    port: u16,
}

impl ServeCommand {
    pub async fn run(self) -> Result<()> {
        println!("Starting server on port: {}", self.port);
        Ok(())
    }
}
