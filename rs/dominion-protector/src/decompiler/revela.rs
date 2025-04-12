use std::{fs::File, io::Write, str::from_utf8};

use anyhow::{bail, Result};
use sqlx::{Acquire, Postgres};
use sui_sdk::types::base_types::ObjectID;
use tempfile::tempdir;
use tokio::{fs, process::Command};

use crate::db::sources::ModuleSource;

pub async fn decompile_module_with_revela_cli<'a, A>(
    db: A,
    network: &str,
    package_id: ObjectID,
    module_name: &str,
    module_bytecode: &[u8],
) -> Result<ModuleSource>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut db = db.acquire().await?;
    // let binary = CompiledModule::deserialize_with_defaults(&module_bytecode)?;
    let dir = tempdir()?;
    let file_path = dir.path().join(format!("{}.mv", module_name));
    let mut file = File::create(&file_path)?;
    file.write_all(&module_bytecode)?;
    let source = Command::new("revela")
        .arg("-b")
        .arg(&file_path)
        .output()
        .await?;
    fs::remove_file(file_path).await?;

    if source.status.success() {
        let sources = ModuleSource {
            package_id: package_id.to_string(),
            module_name: module_name.to_string(),
            network: network.to_string(),
            source: from_utf8(&source.stdout)?.to_string(),
            kind: "revela".to_string(),
        };
        sources.save(&mut *db).await?;
        Ok(sources)
    } else {
        bail!(
            "Failed to decompile module {}: {}",
            module_name,
            from_utf8(&source.stderr)?
        );
    }
}
