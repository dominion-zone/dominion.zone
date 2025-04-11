use std::{
    cell::{RefCell, RefMut},
    collections::BTreeMap,
    ops::DerefMut,
    str::FromStr,
};

use clap::{Args, Subcommand};
use move_binary_format::CompiledModule;
use move_core_types::account_address::AccountAddress;
use move_model::compiled_model::Model;
use sqlx::PgConnection;
use sui_sdk::{
    rpc_types::{SuiObjectData, SuiObjectDataOptions, SuiRawData},
    types::base_types::ObjectID,
};

use crate::{
    db::{
        full_object::{load_object, save_rpc_object},
        Db,
    },
    sui_client::SuiClientWithNetwork,
};
use anyhow::{bail, Result};

#[derive(Args)]
pub struct DownloadCommand {
    #[command(subcommand)]
    command: DownloadType,
}

#[derive(Subcommand)]
enum DownloadType {
    Object { id: String },
    Transaction { digest: String },
}

impl DownloadCommand {
    pub async fn run(self) -> Result<()> {
        let client = SuiClientWithNetwork::with_default_network().await?;
        let mut db = Db::new().await?;
        match self.command {
            DownloadType::Object { id } => {
                // let worker = Uuid::new_v4();
                let object_id = ObjectID::from_str(&id)?;
                println!("Downloading object with ID: {}", &object_id);
                download_object(object_id, &client, &mut db).await?;
                Ok(())
            }
            DownloadType::Transaction { digest } => {
                todo!("Downloading transaction with digest: {}", digest);
            }
        }
    }
}

pub async fn download_object(
    object_id: ObjectID,
    client: &SuiClientWithNetwork,
    db: &Db,
) -> Result<SuiObjectData> {
    let object = client
        .client
        .read_api()
        .get_object_with_options(
            object_id,
            SuiObjectDataOptions {
                show_type: true,
                show_owner: true,
                show_previous_transaction: false, // TODO
                show_display: false,
                show_content: true,
                show_bcs: true,
                show_storage_rebate: false,
            },
        )
        .await?;
    let data = object.data.unwrap();

    // Test code
    /*
    {
        let old = load_object(&db.pool, &client.network, &object_id).await?;
        if let Some(old) = old {
            if old.version == data.version {
                assert_eq!(old.digest, data.digest);
                assert_eq!(old.type_, data.type_);
                assert_eq!(old.owner, data.owner);
                if let Some(SuiRawData::Package(old)) = old.bcs.as_ref() {
                    assert!(matches!(data.bcs.as_ref().unwrap(), SuiRawData::Package(_)));
                    if let SuiRawData::Package(mut data) = data.bcs.clone().unwrap() {
                        assert_eq!(old.id, data.id);
                        assert_eq!(old.module_map, data.module_map);
                        data.type_origin_table.sort();
                        assert_eq!(old.type_origin_table, data.type_origin_table);
                        assert_eq!(old.linkage_table, data.linkage_table);
                    }
                } else {
                    assert_eq!(old.bcs, data.bcs);
                }
                return Ok(data);
            }
            return Ok(data);
        }
    }
    */

    let mut tx = db.pool.begin().await?;
    save_rpc_object(&mut *tx, &client.network, &data).await?;
    tx.commit().await?;
    Ok(data)
}

pub async fn get_or_download_object(
    object_id: ObjectID,
    client: &SuiClientWithNetwork,
    db: &Db,
) -> Result<SuiObjectData> {
    let object = load_object(&db.pool, &client.network, &object_id).await?;
    Ok(if let Some(object) = object {
        object
    } else {
        download_object(object_id, client, db).await?
    })
}

pub async fn get_or_download_model(
    package_id: ObjectID,
    client: &SuiClientWithNetwork,
    db: &Db,
) -> Result<Model> {
    let mut modules = Vec::<CompiledModule>::new();
    let mut unresolved_dependenices = vec![package_id];
    while !unresolved_dependenices.is_empty() {
        let dependency_id = unresolved_dependenices.pop().unwrap();
        let dependency = get_or_download_object(dependency_id, client, db).await?;
        if let SuiRawData::Package(dependency) = dependency.bcs.unwrap() {
            for bytecode in dependency.module_map.values() {
                let compiled = CompiledModule::deserialize_with_defaults(&bytecode)?;
                modules.push(compiled);
            }
            unresolved_dependenices.extend(
                dependency
                    .linkage_table
                    .keys()
                    .filter(|dependency| {
                        AccountAddress::from(**dependency) != AccountAddress::ONE
                            && AccountAddress::from(**dependency) != AccountAddress::TWO
                    })
                    .map(Clone::clone),
            );
        }
    }
    let model = Model::from_compiled(&BTreeMap::new(), modules);

    Ok(model)
}
