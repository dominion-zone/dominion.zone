use std::str::FromStr;

use anyhow::{bail, Result};
use clap::Args;
use sui_sdk::{rpc_types::SuiRawData, types::base_types::ObjectID};

use crate::{
    commands::download::get_or_download_object, db::build_db, sui_client::SuiClientWithNetwork,
};

use super::download::get_or_download_binary_model;

#[derive(Args)]
pub struct ExperimentCommand {
    id: String,
}

impl ExperimentCommand {
    pub async fn run(self) -> Result<()> {
        let client = SuiClientWithNetwork::with_default_network().await?;
        let mut db = build_db().await?;
        let object_id = ObjectID::from_str(&self.id)?;
        let model = get_or_download_binary_model(object_id, &client, &mut db).await?;
        let mut targets = FunctionTargetsHolder::default();
        targets
        Ok(())
    }
}
