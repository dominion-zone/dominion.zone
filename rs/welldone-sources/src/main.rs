use std::collections::HashMap;

use anyhow::Result;
use chrono::{DateTime, Utc};
use clap::Parser;
use dominion_protector::db::{Db, sources::ModuleSource};
use serde::{Deserialize, Serialize};
use sqlx::{Acquire, Postgres};

#[derive(Parser)]
#[command(name = "welldone-sources")]
#[command(about = "Welldone Sources")]
pub struct Cli {
    network: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiPackageDto {
    pub id: i64,
    pub chain_id: String,
    pub account: Option<String>,
    pub package_id: String,
    pub package_name: Option<String>,
    pub is_verified: bool,
    pub verified_src_url: Option<String>,
    pub is_remix_src_uploaded: Option<bool>,
    pub compiled_at: DateTime<Utc>,
    pub deployed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SuiModuleSourceCodeQueryResultDto {
    is_success: bool,
    err_msg: String,
    source_codes: HashMap<String, String>,
}

impl Cli {
    async fn process_package<'a, A>(&self, db: A, id: &str) -> Result<()>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut db = db.acquire().await?;
        let sources = reqwest::get(format!(
            "https://api.welldonestudio.io/compiler//sui/verifications/module-sources/{}/{}",
            &self.network, id,
        ))
        .await?
        .json::<SuiModuleSourceCodeQueryResultDto>()
        .await?;
        for (module_name, source) in sources.source_codes {
            ModuleSource {
                package_id: id.to_string(),
                network: self.network.clone(),
                module_name,
                source,
                kind: "welldone".to_string(),
            }
            .save(&mut *db)
            .await?;
        }

        Ok(())
    }
    pub async fn run(self) -> Result<()> {
        let db = Db::new().await?;

        let mut offset = 0;
        loop {
            let packages = reqwest::get(format!(
                "https://api.welldonestudio.io/compiler/sui/packages?isVerified=true&chainId={}&fetchSize=50&offset={}",
                &self.network, offset
            ))
            .await?
            .json::<Vec<SuiPackageDto>>()
            .await?;
            if packages.is_empty() {
                break;
            }
            for package in &packages {
                println!("Package: {}", &package.package_id);
                let mut tx: sqlx::Transaction<'_, Postgres> = db.pool.begin().await?;
                self.process_package(&mut tx, &package.package_id).await?;
                tx.commit().await?;
            }
            offset += 50;
        }
        // println!("Packages: {:#?}", packages);
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    cli.run().await
}
