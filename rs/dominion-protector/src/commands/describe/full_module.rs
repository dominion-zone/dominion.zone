use crate::{
    ai::AI,
    commands::{
        decompile::get_or_decompile_module, describe::structure, download::get_or_download_object,
    },
    db::{
        descriptions::{create_description_tables_if_needed, FullModuleDescription},
        sources::ModuleSource,
    },
    sui_client::SuiClientWithNetwork,
};
use anyhow::{bail, Context, Result};
use move_binary_format::CompiledModule;
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};

use openai_dive::v1::resources::chat::{ChatMessage, ChatMessageContent};
use sui_sdk::rpc_types::SuiRawData;
use tokio_postgres::Client;

use super::module;

pub async fn describe(
    compiled: &CompiledModule,
    module: &ModuleSource,
    client: &SuiClientWithNetwork,
    db: &mut Client,
    ai: &AI,
) -> Result<FullModuleDescription> {
    create_description_tables_if_needed(db).await?;

    Box::pin(async move {
        let mut input_message = String::new();
        for dependency in &module.dependencies {
            if *dependency.address() == AccountAddress::ONE
                || *dependency.address() == AccountAddress::TWO
            {
                continue;
            }
            let description = get_or_describe(dependency.clone(), db, client, ai).await?;
            input_message.push_str(&format!(
                "*** Dependency {} ***\n  - description ```{}```\n",
                &description.module.id.to_canonical_display(true),
                &description.module.description
            ));
            input_message.push_str("  - structs:\n");
            for struct_description in &description.structs {
                input_message.push_str(&format!(
                    "    - Struct {} description ```{}```\n",
                    struct_description.struct_name, struct_description.description,
                ));
            }
            input_message.push_str("  - functions\n");
            for function_description in &description.functions {
                input_message.push_str(&format!(
                    "    - function {} description ```{}```\n",
                    function_description.function.function_name,
                    function_description.function.description,
                ));
            }
        }
        input_message.push_str(&format!(
            "*** Decomplied module for audit ***: ```move {}```\n",
            &module.source
        ));
        let base_messages = vec![
            ChatMessage::Developer {
                content: ChatMessageContent::Text(ai.prompts.developer.clone()),
                name: None,
            },
            ChatMessage::User {
                content: ChatMessageContent::Text(input_message),
                name: None,
            },
        ];

        let module_info = module::generate(
            module.id.clone(),
            client.network.clone(),
            &mut base_messages.clone(),
            ai,
        )
        .await?;
        let mut structs = vec![];
        for def in compiled.struct_defs() {
            let handle = compiled.datatype_handle_at(def.struct_handle);
            let struct_name = compiled.identifier_at(handle.name).as_str();
            println!("Struct name: {:?}", struct_name);
            structs.push(
                structure::generate(
                    module.id.clone(),
                    client.network.clone(),
                    struct_name,
                    &mut base_messages.clone(),
                    ai,
                )
                .await?,
            );
        }
        let result = FullModuleDescription {
            module: module_info,
            structs,
            functions: vec![], // TODO
        };
        let mut tx = db.transaction().await?;
        result.save_to_db(&mut tx).await?;
        tx.commit().await?;
        Ok(result)
    })
    .await
}

pub async fn get_or_describe(
    module_id: ModuleId,
    db: &mut Client,
    client: &SuiClientWithNetwork,
    ai: &AI,
) -> Result<FullModuleDescription> {
    create_description_tables_if_needed(db).await?;
    let module =
        FullModuleDescription::read_from_db(module_id.clone(), client.network.clone(), db).await?;
    if let Some(module) = module {
        Ok(module)
    } else {
        let package =
            get_or_download_object(module_id.address().clone().into(), client, db).await?;
        if let Some(SuiRawData::Package(package)) = package.bcs {
            let compiled = CompiledModule::deserialize_with_defaults(
                &package
                    .module_map
                    .get(module_id.name().as_str())
                    .context("Can not file module")?,
            )?;
            let module = get_or_decompile_module(module_id, client, db).await?;
            describe(&compiled, &module, client, db, ai).await
        } else {
            bail!("Wrong package id")
        }
    }
}
