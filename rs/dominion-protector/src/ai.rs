use std::ops::Deref;

use openai_dive::v1::{
    api::Client,
    resources::chat::{
        ChatCompletionParameters, ChatCompletionParametersBuilder, ChatMessage, ChatMessageContent,
    },
};

use crate::prompts::Prompts;
use anyhow::{bail, Result};

pub struct AI {
    pub client: Client,
    pub prompts: Prompts,
    pub model: String,
}

impl Deref for AI {
    type Target = Client;

    fn deref(&self) -> &Self::Target {
        &self.client
    }
}

impl AI {
    pub async fn new() -> Result<Self> {
        let ai_api_key = std::env::var("AI_API_KEY").expect("AI_API_KEY is not set");

        let prompts = Prompts::load().await?;

        let mut client = Client::new(ai_api_key);
        client.set_base_url("https://api.atoma.network/v1");

        Ok(Self {
            client,
            prompts,
            model: "deepseek-ai/DeepSeek-R1".to_owned(),
        })
    }

    pub async fn text_request(
        &self,
        params: ChatCompletionParameters,
    ) -> Result<(String, ChatMessage)> {
        loop {
            let result = self.chat().create(params.clone()).await?;
            if let ChatMessage::Assistant { content, .. } = &result.choices[0].message {
                let response: String = match content.as_ref() {
                    Some(ChatMessageContent::Text(text)) => text.clone(),
                    _ => continue,
                };
                let response = if response.starts_with("<think>") {
                    if let Some(end) = response.find("</think>") {
                        response[end + 8..].to_string()
                    } else {
                        println!("No </think> tag in {:?}", result);
                        continue;
                    }
                } else {
                    response
                };
                return Ok((response, result.choices[0].message.clone()));
            } else {
                bail!("Invalid response");
            }
        }
    }
}
