use clap::Args;
use anyhow::Result;

use crate::db::{build_db, clear_db};

#[derive(Args)]
pub struct ClearCommand {}

impl ClearCommand {
    pub async fn run(self) -> Result<()> {
        let mut db = build_db().await?;
        clear_db(&mut db).await?;
        Ok(())
    }
}
