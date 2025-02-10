use clap::{Parser, Subcommand};

pub mod commands;
pub mod db;
pub mod sui_client;
pub mod prompts;
pub mod ai;

use anyhow::Result;
use commands::*;


#[derive(Parser)]
#[command(name = "cli_tool")]
#[command(about = "A command-line utility for various operations")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Download(download::DownloadCommand),
    Decompile(decompile::DecompileCommand),
    Describe(describe::DescribeCommand),
    Serve(serve::ServeCommand),
    Watch(watch::WatchCommand),
    Clear(clear::ClearCommand),
}

impl Cli {
    pub async fn run(self) -> Result<()> {
        match self.command {
            Commands::Download(cmd) => cmd.run().await,
            Commands::Decompile(cmd) => cmd.run().await,
            Commands::Describe(cmd) => cmd.run().await,
            Commands::Serve(cmd) => cmd.run().await,
            Commands::Watch(cmd) => cmd.run().await,
            Commands::Clear(cmd) => cmd.run().await,
        }
    }
}
