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
            SecurityLevel, StructDescription,
        },
        sources::{read_source_from_db, ModuleSource},
    },
    prompts::Prompts,
    sui_client::SuiClientWithNetwork,
};
use anyhow::{bail, Context, Result};
use clap::{Args, Subcommand};
use move_core_types::{account_address::AccountAddress, language_storage::ModuleId};
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    JsonSchema, JsonSchemaBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sui_sdk::{
    rpc_types::SuiRawData,
    types::{base_types::ObjectID, Identifier},
};
use tokio_postgres::Client;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Ownership {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_owned: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object_owned: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shared: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub immutable: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event: Option<String>,
}

async fn generate_ownership(
    struct_name: &str,
    messages: &mut Vec<ChatMessage>,
    ai: &AI,
) -> Result<Ownership> {
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(
            ai.prompts
                .structure
                .ownership
                .replace("{struct_name}", struct_name),
        ),
        name: None,
    });

    let parameters = ChatCompletionParametersBuilder::default()
        .model(ai.model.clone())
        .messages(messages.as_slice())
        .response_format(ChatCompletionResponseFormat::JsonSchema(
            JsonSchemaBuilder::default()
                .description("Ownership conditions")
                .name("Ownership")
                .schema(json!({
                    "$schema": "https://json-schema.org/draft/2020-12/schema",
                    "type": "object",
                    "properties": {
                        "address_owned": {
                            "type": "string",
                            "description": "Indicates if the struct may be owned by a user. 'always' means it is always user-owned, null means it is not."
                        },
                        "object_owned": {
                            "type": "string",
                            "description": "Indicates if the struct may be owned by another object. 'always' means it is always object-owned, null means it is not."
                        },
                        "wrapped": {
                            "type": "string",
                            "description": "Indicates if the struct may be wrapped inside another struct. Possible values: 'always', specific conditions (as a string), or null if not applicable."
                        },
                        "shared": {
                            "type": "string",
                            "description": "Indicates if the struct may be shared across multiple users. 'always' means it is always shared, null means it is not."
                        },
                        "immutable": {
                            "type": "string",
                            "description": "Indicates if the struct is immutable. 'always' means it is always immutable, null means it is not."
                        }
                    },
                    "additionalProperties": false,
                }))
                .strict(true)
                .build()?,
        ))
        .build()?;

    let result = ai.chat().create(parameters).await?;
    if let ChatMessage::Assistant { content, .. } = &result.choices[0].message {
        let response: String = match content.as_ref() {
            Some(ChatMessageContent::Text(text)) => text.clone(),
            _ => bail!("Invalid response"),
        };
        println!("Response: {:?}", result.choices[0]);
        let ownership: Ownership = serde_json::from_str(&response)?;
        messages.push(result.choices[0].message.clone());

        Ok(ownership)
    } else {
        bail!("Invalid response")
    }
}

async fn generate_description(
    struct_name: &str,
    messages: &mut Vec<ChatMessage>,
    ai: &AI,
) -> Result<String> {
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(
            ai.prompts
                .structure
                .description
                .replace("{struct_name}", struct_name),
        ),
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
            println!("Response: {:?}", choice);
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

async fn generate_warnings(
    struct_name: &str,
    messages: &mut Vec<ChatMessage>,
    ai: &AI,
) -> Result<Vec<String>> {
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(
            ai.prompts
                .structure
                .warnings
                .replace("{struct_name}", struct_name),
        ),
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
            println!("Response: {:?}", choice);
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

pub async fn generate(
    module: ModuleId,
    network: String,
    structure_name: &str,
    messages: &Vec<ChatMessage>,
    ai: &AI,
) -> Result<StructDescription> {
    let mut messages = messages.clone();
    let Ownership {
        address_owned,
        object_owned,
        wrapped,
        shared,
        immutable,
        event,
    } = generate_ownership(structure_name, &mut messages, ai).await?;
    let description = generate_description(structure_name, &mut messages, ai).await?;
    let warnings = generate_warnings(structure_name, &mut messages, ai).await?;

    Ok(StructDescription {
        module,
        network,
        struct_name: structure_name.to_owned(),
        description,
        address_owned,
        object_owned,
        wrapped,
        shared,
        immutable,
        event,
        warnings,
    })
}
