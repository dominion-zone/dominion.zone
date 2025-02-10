use std::{
    collections::BTreeMap,
    fs::File,
    io::Write,
    str::{from_utf8, FromStr},
};

use anyhow::{bail, Context, Result};
use clap::Args;
use move_binary_format::CompiledModule;
use move_core_types::language_storage::ModuleId;
use sui_sdk::{
    rpc_types::{SuiRawData, SuiRawMovePackage},
    types::{base_types::ObjectID, Identifier},
};
use tempfile::tempdir;
use tokio::{fs, process::Command};
use tokio_postgres::Client;

use crate::{
    commands::download::{get_or_download_object, DownloadObjectParams},
    db::{
        build_db,
        sources::{create_sources_tables_if_needed, read_source_from_db, ModuleSource},
    },
    sui_client::SuiClientWithNetwork,
};

#[derive(Args)]
pub struct DecompileCommand {
    id: String,
}

impl DecompileCommand {
    pub async fn run(self) -> Result<()> {
        let client = SuiClientWithNetwork::with_default_network().await?;
        let mut db = build_db().await?;
        create_sources_tables_if_needed(&db).await?;
        let object_id = ObjectID::from_str(&self.id)?;
        println!("Decompiling package with ID: {}", object_id);
        let package = get_or_download_object(DownloadObjectParams {
            object_id,
            client: &client,
            db: &mut db,
        })
        .await?;
        if let SuiRawData::Package(package) = package.bcs.unwrap() {
            let _ = decompile(DecompileParams {
                network: client.network.clone(),
                package: &package,
                db: &mut db,
            })
            .await?;
        } else {
            bail!("Object is not a package");
        }
        Ok(())
    }
}

pub struct DecompileParams<'a> {
    pub network: String,
    pub package: &'a SuiRawMovePackage,
    pub db: &'a mut Client,
}

pub async fn decompile(
    DecompileParams {
        package,
        network,
        db,
    }: DecompileParams<'_>,
) -> Result<BTreeMap<Identifier, ModuleSource>> {
    let mut modules = BTreeMap::<Identifier, ModuleSource>::new();
    let tx = db.transaction().await?;
    for (name, bytecode) in &package.module_map {
        let binary = CompiledModule::deserialize_with_defaults(&bytecode)?;
        let dependencies: Vec<_> = binary
            .immediate_dependencies()
            .into_iter()
            .map(|d| d.to_canonical_string(true))
            .collect();
        let dir = tempdir()?;
        let file_path = dir.path().join(format!("{}.mv", name));
        let mut file = File::create(&file_path)?;
        file.write_all(&bytecode)?;
        let source = Command::new("revela")
            .arg("-b")
            .arg(&file_path)
            .output()
            .await?;
        fs::remove_file(file_path).await?;
        let name_id = Identifier::from_str(&name)?;
        modules.insert(
            name_id.clone(),
            ModuleSource {
                id: ModuleId::new(package.id.into(), name_id),
                network: network.clone(),
                source: from_utf8(&source.stdout)?.to_string(),
                kind: "revela".to_string(),
                dependencies: dependencies
                    .iter()
                    .map(|d| ModuleId::from_str(&d))
                    .collect::<Result<Vec<_>>>()?,
            },
        );
        if source.status.success() {
            tx.execute(
                "INSERT INTO module_sources(
                    package_id,
                    network,
                    name,
                    source,
                    kind,
                    dependencies)
                VALUES($1, $2, $3, $4, 'revela', $5)",
                &[
                    &package.id.to_string(),
                    &network,
                    &name,
                    &from_utf8(&source.stdout)?,
                    &dependencies,
                ],
            )
            .await?;
        } else {
            bail!(
                "Failed to decompile module {}: {}",
                name,
                from_utf8(&source.stderr)?
            );
        }
    }
    tx.commit().await?;
    Ok(modules)
}

pub async fn get_or_decompile_module(
    module_id: ModuleId,
    client: &SuiClientWithNetwork,
    db: &mut Client,
) -> Result<ModuleSource> {
    let source = read_source_from_db(module_id.clone(), client.network.clone(), db).await?;
    if let Some(source) = source {
        Ok(source)
    } else {
        let object_id: ObjectID = module_id.address().clone().into();
        let package = get_or_download_object(DownloadObjectParams {
            object_id,
            client,
            db,
        })
        .await?;
        if let SuiRawData::Package(package) = package.bcs.unwrap() {
            let modules = decompile(DecompileParams {
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
