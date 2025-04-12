use std::str::FromStr;

use anyhow::{bail, Result};
use clap::Args;
use sui_sdk::{rpc_types::SuiRawData, types::base_types::ObjectID};

use crate::{commands::download::get_or_download_object, db::Db, sui_client::SuiClientWithNetwork};

use super::download::get_or_download_model;

#[derive(Args)]
pub struct ExperimentCommand {
    id: String,
}

impl ExperimentCommand {
    pub async fn run(self) -> Result<()> {
        let client = SuiClientWithNetwork::with_default_network().await?;
        let db = Db::new().await?;
        let object_id = ObjectID::from_str(&self.id)?;
        let package = get_or_download_object(&object_id, &client, &db).await?;
        if let Some(SuiRawData::Package(package)) = package.bcs {
        } else {
            bail!("Expected package, got {:?}", package);
        }
        Ok(())
    }
}
