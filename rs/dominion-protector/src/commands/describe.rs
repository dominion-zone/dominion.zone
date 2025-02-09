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
            self, create_description_tables_if_needed, FullModuleDescription, ModuleDescription,
            SecurityLevel,
        },
        sources::{read_source_from_db, ModuleSource},
    },
    prompts::Prompts,
    sui_client::{build_client, SuiClientWithNetwork},
};
use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};
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
        let client = build_client().await?;
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
                    for (name, _) in &package.module_map {
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
                        describe_module(module, &client, &mut db, &ai).await?;
                    }
                } else {
                    bail!("Object is not a package");
                }
                Ok(())
            }
            DescribeType::Module { full_name } => {
                let module_id = ModuleId::from_str(&full_name)?;
                println!("Describing module: {}", &module_id);
                let module = get_or_decompile_module(module_id, &client, &mut db).await?;
                describe_module(module, &client, &mut db, &ai).await?;
                Ok(())
            }
            DescribeType::Transaction { digest } => {
                println!("Describing transaction with digest: {}", digest);
                Ok(())
            }
        }
    }
}

async fn generate_module_description(messages: &mut Vec<ChatMessage>, ai: &AI) -> Result<String> {
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(ai.prompts.module.description.clone()),
        name: None,
    });

    let parameters = ChatCompletionParametersBuilder::default()
        .model(ai.model.clone())
        .messages(messages.as_slice())
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let result = ai.chat().create(parameters).await?;
    let mut description: Option<String> = None;
    for choice in result.choices {
        if let ChatMessage::Assistant { content, .. } = &choice.message {
            let response: String = match content.as_ref() {
                Some(ChatMessageContent::Text(text)) => text.clone(),
                Some(ChatMessageContent::ContentPart(_)) => continue,
                Some(ChatMessageContent::None) => continue,
                None => continue,
            };
            println!("Response: {}", response);
            let response = if response.starts_with("<think>") {
                let end = response.find("</think>").context("No </think> tag")?;
                response[end + 8..].to_string()
            } else {
                response
            };
            description.replace(response);
            messages.push(choice.message);
            break;
        }
    }

    Ok(description.context("Error getting description")?)
}

async fn generate_module_warnings(messages: &mut Vec<ChatMessage>, ai: &AI) -> Result<Vec<String>> {
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(ai.prompts.module.warnings.clone()),
        name: None,
    });

    let parameters = ChatCompletionParametersBuilder::default()
        .model(ai.model.clone())
        .messages(messages.as_slice())
        .response_format(ChatCompletionResponseFormat::JsonSchema(
            JsonSchemaBuilder::default()
                .description("Array of strings")
                .name("String[]")
                .schema(json!({
                  "$schema": "https://json-schema.org/draft/2020-12/schema",
                  "type": "array",
                  "items": {
                    "type": "string"
                  },
                  "example": ["A", "B"]
                }))
                .strict(true)
                .build()?,
        ))
        .build()?;

    let result = ai.chat().create(parameters).await?;
    let mut wranings: Option<Vec<String>> = None;

    for choice in result.choices {
        if let ChatMessage::Assistant { content, .. } = &choice.message {
            let response: String = match content.as_ref() {
                Some(ChatMessageContent::Text(text)) => text.clone(),
                Some(ChatMessageContent::ContentPart(_)) => continue,
                Some(ChatMessageContent::None) => continue,
                None => continue,
            };
            println!("Response: {}", response);
            let result = serde_json::from_str(&response);
            if let Ok(result) = result {
                wranings.replace(result);
                messages.push(choice.message);
                break;
            }
        }
    }

    Ok(wranings.context("Error getting warnings")?)
}

async fn generate_module_security_level(
    messages: &mut Vec<ChatMessage>,
    ai: &AI,
) -> Result<SecurityLevel> {
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(ai.prompts.module.security_level.clone()),
        name: None,
    });

    let parameters = ChatCompletionParametersBuilder::default()
        .model(ai.model.clone())
        .messages(messages.as_slice())
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let result = ai.chat().create(parameters).await?;
    let mut security_level: Option<SecurityLevel> = None;
    for choice in result.choices {
        if let ChatMessage::Assistant { content, .. } = &choice.message {
            let response: String = match content.as_ref() {
                Some(ChatMessageContent::Text(text)) => text.clone(),
                Some(ChatMessageContent::ContentPart(_)) => continue,
                Some(ChatMessageContent::None) => continue,
                None => continue,
            };
            let response = if response.starts_with("<think>") {
                let end = response.find("</think>").context("No </think> tag")?;
                response[end + 8..].to_string()
            } else {
                response
            };
            println!("Response: {}", response);
            if response.contains("Critical") {
                security_level.replace(SecurityLevel::CriticalRisk);
            } else if response.contains("High") {
                security_level.replace(SecurityLevel::HighRisk);
            } else if response.contains("Medium") {
                security_level.replace(SecurityLevel::MediumRisk);
            } else if response.contains("Low") {
                security_level.replace(SecurityLevel::LowRisk);
            } else if response.contains("Best") {
                security_level.replace(SecurityLevel::BestPracticesCompliant);
            } else if response.contains("Unknown") {
                security_level.replace(SecurityLevel::UnknownUnassessed);
            } else {
                continue;
            }
            messages.push(choice.message);
            break;
        }
    }

    Ok(security_level.context("Error getting security_level")?)
}

async fn generate_module_info(
    id: ModuleId,
    network: String,
    messages: &mut Vec<ChatMessage>,
    ai: &AI,
) -> Result<ModuleDescription> {
    // TODO: FullModuleDescription
    let description = generate_module_description(messages, ai).await?;
    let warnings = generate_module_warnings(messages, ai).await?;
    let security_level = generate_module_security_level(messages, ai).await?;

    Ok(ModuleDescription {
        id,
        network,
        description,
        warnings,
        security_level,
    })
}

async fn describe_module(
    module: ModuleSource,
    client: &SuiClientWithNetwork,
    db: &mut Client,
    ai: &AI,
) -> Result<ModuleDescription> {
    // TODO FullModuleDescription
    Box::pin(async move {
        let mut input_message = String::new();
        for dependency in &module.dependencies {
            if *dependency.address() == AccountAddress::ONE
                || *dependency.address() == AccountAddress::TWO
            {
                continue;
            }
            let description = get_or_describe_module(dependency.clone(), db, client, ai).await?;
            input_message.push_str(&format!(
                "Dependency {} description ```{}```\n",
                &description.id.to_canonical_display(true),
                &description.description
            ));
        }
        input_message.push_str(&format!(
            "Decomplied module for audit: ```move {}```\n",
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

        let module_info = generate_module_info(
            module.id.clone(),
            client.network.clone(),
            &mut base_messages.clone(),
            ai,
        )
        .await?;
        let mut tx = db.transaction().await?;
        module_info.save_to_db(&mut tx).await?;
        tx.commit().await?;
        Ok(module_info)
    })
    .await
}

pub async fn get_or_describe_module(
    module_id: ModuleId,
    db: &mut Client,
    client: &SuiClientWithNetwork,
    ai: &AI,
) -> Result<ModuleDescription> {
    let module =
        ModuleDescription::read_from_db(module_id.clone(), client.network.clone(), db).await?;
    if let Some(module) = module {
        Ok(module)
    } else {
        let module = get_or_decompile_module(module_id, client, db).await?;
        describe_module(module, client, db, ai).await
    }
}
