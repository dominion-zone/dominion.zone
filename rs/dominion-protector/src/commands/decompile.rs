use std::{
    collections::BTreeMap,
    fmt::Display,
    fs::File,
    io::Write,
    str::{from_utf8, FromStr},
};

use anyhow::{bail, Context, Result};
use clap::Args;
use move_binary_format::CompiledModule;
use move_bytecode_utils::Modules;
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
    decompiler::{
        decompile_module_to_smt, decompile_module_with_disasm,
        revela::decompile_module_with_revela_cli,
    },
    sui_client::SuiClientWithNetwork,
};

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Kind {
    Revela,
    Disassembled,
}

impl Display for Kind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Kind::Revela => write!(f, "revela"),
            Kind::Disassembled => write!(f, "disassembled"),
        }
    }
}

#[derive(Args)]
pub struct DecompileCommand {
    pub id: Option<String>,
    #[arg(long, default_value = "disassembled")]
    pub kind: Kind,
}

impl DecompileCommand {
    pub async fn decompile_package<'a, A>(
        &self,
        db: A,
        network: &str,
        package: &SuiRawMovePackage,
    ) -> Result<()>
    // Result<BTreeMap<Identifier, ModuleSource>>
    where
        A: Acquire<'a, Database = Postgres>,
    {
        let mut db = db.acquire().await?;
        match self.kind {
            Kind::Revela => {
                for (module_name, module_bytecode) in package.module_map.iter() {
                    let _ = decompile_module_with_revela_cli(
                        &mut *db,
                        network,
                        package.id,
                        &module_name,
                        module_bytecode,
                    )
                    .await?;
                }
            }
            Kind::Disassembled => {
                let compiled = package
                    .module_map
                    .values()
                    .map(|m| Ok(CompiledModule::deserialize_with_defaults(m)?))
                    .collect::<Result<Vec<_>>>()?;
                let all_modules = Modules::new(compiled.iter());
                for module in all_modules.compute_topological_order()? {
                    let _ =
                        decompile_module_with_disasm(&mut *db, network, package.id, module).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn process_package(
        &self,
        client: &SuiClientWithNetwork,
        db: &Db,
        object_id: &ObjectID,
    ) -> Result<()> {
        println!("Decompiling package with ID: {}", object_id);
        let package = get_or_download_object(object_id, &client, db).await?;
        if let SuiRawData::Package(package) = package.bcs.unwrap() {
            let mut tx = db.pool.begin().await?;
            let _ = self
                .decompile_package(&mut *tx, &client.network, &package)
                .await?;
            tx.commit().await?;
        } else {
            bail!("Object is not a package");
        }
        Ok(())
    }

    pub async fn process_all_packages(&self, client: &SuiClientWithNetwork, db: &Db) -> Result<()> {
        let ids = query!(
            "SELECT
                object_id
            FROM objects
            LEFT JOIN module_sources ON
                objects.object_id = module_sources.package_id AND
                module_sources.network = objects.network AND
                module_sources.kind = $1
            WHERE
                objects.object_type = 'package' AND
                objects.network = $2 AND
                module_sources.package_id IS NULL",
            self.kind.to_string(),
            &client.network,
        )
        .fetch_all(&db.pool)
        .await?;
        for id in ids {
            self.process_package(&client, db, &ObjectID::from_str(&id.object_id)?)
                .await?;
        }
        Ok(())
    }

    pub async fn run(self) -> Result<()> {
        let client = SuiClientWithNetwork::with_default_network().await?;
        let mut db = Db::new().await?;
        if let Some(id) = &self.id {
            self.process_package(&client, &mut db, &ObjectID::from_str(&id)?)
                .await?;
        } else {
            self.process_all_packages(&client, &mut db).await?;
        }

        Ok(())
    }
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
