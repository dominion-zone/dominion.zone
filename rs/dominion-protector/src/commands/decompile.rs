use std::{
    collections::BTreeMap,
    fs::File,
    io::Write,
    str::{from_utf8, FromStr},
};

use anyhow::{bail, Context, Result};
use clap::Args;
use move_binary_format::CompiledModule;
use move_bytecode_utils::Modules;
use move_core_types::language_storage::ModuleId;
use sqlx::{query, Acquire, Postgres};
use sui_sdk::{
    rpc_types::{SuiRawData, SuiRawMovePackage},
    types::{base_types::ObjectID, Identifier},
};
use tempfile::tempdir;
use tokio::{fs, process::Command};

use crate::{
    commands::download::get_or_download_object,
    db::{sources::ModuleSource, Db},
    decompiler::decompile_module_to_smt,
    sui_client::SuiClientWithNetwork,
};

#[derive(Args)]
pub struct DecompileCommand {
    id: String,
}

impl DecompileCommand {
    pub async fn run(self) -> Result<()> {
        let client = SuiClientWithNetwork::with_default_network().await?;
        let mut db = Db::new().await?;
        let object_id = ObjectID::from_str(&self.id)?;
        println!("Decompiling package with ID: {}", object_id);
        let package = get_or_download_object(object_id, &client, &mut db).await?;
        if let SuiRawData::Package(package) = package.bcs.unwrap() {
            let mut tx = db.pool.begin().await?;
            let _ = decompile_package(&mut *tx, &client.network, &package).await?;
            tx.commit().await?;
        } else {
            bail!("Object is not a package");
        }
        Ok(())
    }
}

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

pub async fn decompile_package<'a, A>(
    db: A,
    network: &str,
    package: &SuiRawMovePackage,
) -> Result<()>
// Result<BTreeMap<Identifier, ModuleSource>>
where
    A: Acquire<'a, Database = Postgres>,
{
    // let mut modules = BTreeMap::<Identifier, ModuleSource>::new();
    let compiled = package
        .module_map
        .values()
        .map(|m| Ok(CompiledModule::deserialize_with_defaults(m)?))
        .collect::<Result<Vec<_>>>()?;
    let mut db = db.acquire().await?;
    let all_modules = Modules::new(compiled.iter());
    for module in all_modules.compute_topological_order()? {
        /*
        let name_id = Identifier::from_str(&module_name)?;
        modules.insert(
            name_id,*/
        decompile_module_to_smt(&mut *db, network, package.id, module).await?; /*,
                                                                               );*/
    }
    Ok(())
}

/*
pub async fn get_or_decompile_module(
    module_id: ModuleId,
    client: &SuiClientWithNetwork,
    db: &mut Client,
) -> Result<ModuleSource> {
    create_sources_tables_if_needed(&db).await?;
    let source = read_source_from_db(module_id.clone(), client.network.clone(), db).await?;
    if let Some(source) = source {
        Ok(source)
    } else {
        let object_id: ObjectID = module_id.address().clone().into();
        let package = get_or_download_object(object_id, client, db).await?;
        if let SuiRawData::Package(package) = package.bcs.unwrap() {
            let modules = decompile_package(DecompileParams {
                network: client.network.clone(),
                package: &package,
                db,
            })
            .await?;
            Ok(modules
                .get(module_id.name())
                .context("Module not found")?
                .clone())
        } else {
            bail!("Object is not a package");
        }
    }
}
*/
