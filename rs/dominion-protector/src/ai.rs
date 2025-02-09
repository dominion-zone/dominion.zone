use std::ops::Deref;

use openai_dive::v1::{api::Client, resources::chat::ChatCompletionParametersBuilder};

use crate::prompts::Prompts;
use anyhow::Result;

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
            model: "deepseek-ai/DeepSeek-R1".to_owned()
        })
    }
}