use std::str::FromStr;

use crate::{
    ai::AI,
    commands::{
        decompile::{decompile, get_or_decompile_module, DecompileParams},
        download::{download_object, get_or_download_object, DownloadObjectParams},
    },
    db::{
        build_db,
        descriptions::{
            create_description_tables_if_needed, FullModuleDescription, ModuleDescription,
            SecurityLevel,
        },
        sources::{read_source_from_db, ModuleSource},
    },
    prompts::Prompts,
    sui_client::SuiClientWithNetwork,
};
use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};
use move_binary_format::CompiledModule;
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    JsonSchema, JsonSchemaBuilder,
};
use serde_json::json;
use sui_sdk::{
    rpc_types::SuiRawData,
    types::{base_types::ObjectID, Identifier},
};
use tokio_postgres::Client;

pub mod module;
pub mod full_module;
pub mod structure;

#[derive(Args)]
pub struct DescribeCommand {
    #[command(subcommand)]
    command: DescribeType,
}

#[derive(Subcommand)]
enum DescribeType {
    Package { package_id: String },
    Module { full_name: String },
    Transaction { digest: String },
}

impl DescribeCommand {
    pub async fn run(self) -> Result<()> {
        let client = SuiClientWithNetwork::with_default_network().await?;
        let mut db = build_db().await?;
        let ai = AI::new().await?;

        create_description_tables_if_needed(&mut db).await?;
        match self.command {
            DescribeType::Package { package_id } => {
                println!("Describing package with ID: {}", package_id);
                let object_id = ObjectID::from_str(&package_id)?;
                let package = get_or_download_object(DownloadObjectParams {
                    object_id,
                    client: &client,
                    db: &mut db,
                })
                .await?;
                if let SuiRawData::Package(package) = package.bcs.unwrap() {
                    for (name, bytecode) in &package.module_map {
                        let compiled = CompiledModule::deserialize_with_defaults(&bytecode)?;
                        let module_id =
                            ModuleId::new(object_id.into(), Identifier::new(name.clone())?);
                        let module =
                            read_source_from_db(module_id.clone(), client.network.clone(), &mut db)
                                .await?;
                        let module = if let Some(module) = module {
                            module
                        } else {
                            let modules = decompile(DecompileParams {
                                db: &mut db,
                                network: client.network.clone(),
                                package: &package,
                            })
                            .await?;
                            modules
                                .get(module_id.name())
                                .context("Module not found")?
                                .clone()
                        };
                        full_module::describe(&compiled, &module, &client, &mut db, &ai).await?;
                    }
                } else {
                    bail!("Object is not a package");
                }
                Ok(())
            }
            DescribeType::Module { full_name } => {
                let module_id = ModuleId::from_str(&full_name)?;
                println!("Describing module: {}", &module_id);
                let package = get_or_download_object(DownloadObjectParams {
                    object_id: ObjectID::from_address(module_id.address().clone()),
                    client: &client,
                    db: &mut db,
                })
                .await?;
                if let SuiRawData::Package(package) = package.bcs.unwrap() {
                    let compiled = CompiledModule::deserialize_with_defaults(
                        &package
                            .module_map
                            .get(module_id.name().as_str())
                            .context("Can not file module")?,
                    )?;
                    let module = get_or_decompile_module(module_id, &client, &mut db).await?;
                    full_module::describe(&compiled, &module, &client, &mut db, &ai).await?;
                } else {
                    bail!("Wrong package id")
                }
                Ok(())
            }
            DescribeType::Transaction { digest } => {
                println!("Describing transaction with digest: {}", digest);
                Ok(())
            }
        }
    }
}
