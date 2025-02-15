use crate::{
    ai::AI,
    db::descriptions::{ModuleDescription, SecurityLevel},
};
use anyhow::{bail, Context, Result};
use move_core_types::language_storage::ModuleId;
use openai_dive::v1::resources::chat::{
    ChatCompletionParametersBuilder, ChatCompletionResponseFormat, ChatMessage, ChatMessageContent,
    JsonSchemaBuilder,
};
use serde_json::json;

async fn generate_description(messages: &mut Vec<ChatMessage>, ai: &AI) -> Result<String> {
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(ai.prompts.module.description.clone()),
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

async fn generate_warnings(messages: &mut Vec<ChatMessage>, ai: &AI) -> Result<Vec<String>> {
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

async fn generate_security_level(
    messages: &mut Vec<ChatMessage>,
    ai: &AI,
) -> Result<SecurityLevel> {
    println!("Security level");
    messages.push(ChatMessage::User {
        content: ChatMessageContent::Text(ai.prompts.module.security_level.clone()),
        name: None,
    });

    let parameters = ChatCompletionParametersBuilder::default()
        .model(ai.model.clone())
        .messages(messages.as_slice())
        .response_format(ChatCompletionResponseFormat::Text)
        .build()?;

    let (response, message) = ai.text_request(parameters).await?;
    let security_level = if response.contains("Critical") {
        SecurityLevel::CriticalRisk
    } else if response.contains("High") {
        SecurityLevel::HighRisk
    } else if response.contains("Medium") {
        SecurityLevel::MediumRisk
    } else if response.contains("Low") {
        SecurityLevel::LowRisk
    } else if response.contains("Best") {
        SecurityLevel::BestPracticesCompliant
    } else if response.contains("Unknown") {
        SecurityLevel::UnknownUnassessed
    } else {
        bail!("Invalid response");
    };
    messages.push(message);
    Ok(security_level)
}

pub async fn generate(
    id: ModuleId,
    network: String,
    messages: &mut Vec<ChatMessage>,
    ai: &AI,
) -> Result<ModuleDescription> {
    // TODO: FullModuleDescription
    let description = generate_description(messages, ai).await?;
    let warnings = generate_warnings(messages, ai).await?;
    let security_level = generate_security_level(messages, ai).await?;

    Ok(ModuleDescription {
        id,
        network,
        description,
        warnings,
        security_level,
    })
}
