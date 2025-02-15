use crate::{ai::AI, db::descriptions::StructDescription};
use anyhow::{bail, Context, Result};
use move_core_types::language_storage::ModuleId;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    JsonSchemaBuilder,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

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
                            "description": "Indicates if the struct may be owned by a user. 'always' means it is always user-owned, undefined means never."
                        },
                        "object_owned": {
                            "type": "string",
                            "description": "Indicates if the struct may be owned by another object. 'always' means it is always object-owned, undefined means never."
                        },
                        "wrapped": {
                            "type": "string",
                            "description": "Indicates if the struct may be wrapped inside another struct. Possible values: 'always', specific conditions (as a string), or undefined if not applicable."
                        },
                        "shared": {
                            "type": "string",
                            "description": "Indicates if the struct may be shared across multiple users. 'always' means it is always shared, undefined means never."
                        },
                        "immutable": {
                            "type": "string",
                            "description": "Indicates if the struct is immutable. 'always' means it is always immutable, undefined means never."
                        },
                        "event": {
                            "type": "string",
                            "description": "Indicates if the struct is an event. 'always' means it is always an event, undefined means never."
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

    let (description, message) = ai.text_request(parameters).await?;
    messages.push(message);

    Ok(description)
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
    let (description, warnings) = {
        let mut messages = messages.clone();
        let description = generate_description(structure_name, &mut messages, ai).await?;
        let warnings = generate_warnings(structure_name, &mut messages, ai).await?;
        (description, warnings)
    };

    let Ownership {
        address_owned,
        object_owned,
        wrapped,
        shared,
        immutable,
        event,
    } = {
        let mut messages = messages.clone();
        generate_ownership(structure_name, &mut messages, ai).await?
    };

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
