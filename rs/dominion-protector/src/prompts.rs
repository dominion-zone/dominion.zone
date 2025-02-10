use anyhow::Result;
use serde::{Deserialize, Serialize};
use tokio::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prompts {
    pub developer: String,
    pub module: ModulePrompts,
    pub structure: StructPrompts,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModulePrompts {
    pub description: String,
    pub security_level: String,
    pub warnings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructPrompts {
    pub description: String,
    pub ownership: String,
    pub warnings: String,
}

impl Prompts {
    pub async fn load() -> Result<Self> {
        let contents = fs::read("prompts.yaml").await?;
        Ok(serde_yml::from_slice(&contents)?)
    }
}
