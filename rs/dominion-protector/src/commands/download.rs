use std::str::FromStr;

use clap::{Args, Subcommand};
use sui_sdk::{
    rpc_types::{SuiObjectData, SuiObjectDataOptions, SuiRawData},
    types::{base_types::ObjectID, move_package::TypeOrigin, object::Owner},
};
use tokio_postgres::Client;

use crate::{
    db::{
        build_db,
        objects::{create_objects_tables_if_needed, read_object_from_db, OwnerType},
    },
    sui_client::{build_client, SuiClientWithNetwork},
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
        let client = build_client().await?;
        let mut db = build_db().await?;
        create_objects_tables_if_needed(&db).await?;
        match self.command {
            DownloadType::Object { id } => {
                // let worker = Uuid::new_v4();
                let object_id = ObjectID::from_str(&id)?;
                println!("Downloading object with ID: {}", &object_id);
                /*
                let row = db
                    .execute(
                        "INSERT INTO download_object_tasks(object_id, network, worker)
                    VALUES ($1, $2, $3)
                    ON CONFLICT(object_id, network) DO NOTHING",
                        &[&object_id.to_string(), &client.network, &worker.to_string()],
                    )
                    .await?;
                if row == 0 {
                    let _ = wait_for_object(object_id, client.network.clone(), &mut db).await;
                    return Ok(());
                }
                */
                download_object(DownloadObjectParams {
                    object_id,
                    client: &client,
                    db: &mut db,
                })
                .await?;
                Ok(())
            }
            DownloadType::Transaction { digest } => {
                todo!("Downloading transaction with digest: {}", digest);
            }
        }
    }
}

pub struct DownloadObjectParams<'a> {
    pub object_id: ObjectID,
    pub client: &'a SuiClientWithNetwork,
    pub db: &'a mut Client,
}

pub async fn download_object(
    DownloadObjectParams {
        object_id,
        client,
        db,
    }: DownloadObjectParams<'_>,
) -> Result<SuiObjectData> {
    // let inner = async {
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
    let tx = db.transaction().await?;
    /*
    let rows = tx.execute(
                "DELETE FROM download_object_tasks WHERE object_id = $1 AND network = $2 AND worker = $3",
                &[&object_id.to_string(), &client.network, &worker.to_string()],
            )
            .await?;
    if rows == 0 {
        // Someone kicked the task before us
        // read the object from the database
    }
    */
    let (owner_type, owner, initial_shared_version) = match data.owner.as_ref().unwrap() {
        Owner::AddressOwner(sui_address) => {
            (OwnerType::AddressOwner, Some(sui_address.to_string()), None)
        }
        Owner::ObjectOwner(sui_address) => {
            (OwnerType::ObjectOwner, Some(sui_address.to_string()), None)
        }
        Owner::Shared {
            initial_shared_version,
        } => (OwnerType::Shared, None, Some(initial_shared_version)),
        Owner::Immutable => (OwnerType::Immutable, None, None),
        Owner::ConsensusV2 { .. } => (OwnerType::ConsensusV2, None, None),
    };
    let rows = tx
        .execute(
            "INSERT INTO objects(
            id,
            network,
            version,
            digest,
            type,
            owner_type,
            owner,
            initial_shared_version)
        VALUES($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (id, network) DO UPDATE 
        SET version = excluded.version, 
            digest = excluded.digest,
            type = excluded.type,
            owner_type = excluded.owner_type,
            owner = excluded.owner,
            initial_shared_version = excluded.initial_shared_version",
            &[
                &data.object_id.to_string(),
                &client.network,
                &i64::try_from(data.version.value())?,
                &data.digest.to_string(),
                &data.type_.as_ref().unwrap().to_string(),
                &owner_type,
                &owner,
                &initial_shared_version.and_then(|s| i64::try_from(s.value()).ok()),
            ],
        )
        .await?;
    match data.bcs.as_ref().unwrap() {
        SuiRawData::MoveObject(_) => {
            bail!("MoveObject not supported yet");
        }
        SuiRawData::Package(package) => {
            if rows == 0 {
                // Packages are immutable, so we can skip rewriting the data
                return Ok(data);
            }
            for (name, bytecode) in &package.module_map {
                tx.execute(
                    "INSERT INTO package_modules(
                        package_id,
                        network,
                        name,
                        bytecode)
                    VALUES($1, $2, $3, $4)",
                    &[&package.id.to_string(), &client.network, &name, &bytecode],
                )
                .await?;
            }
            for TypeOrigin {
                module_name,
                // `struct_name` alias to support backwards compatibility with the old name
                datatype_name,
                package: origin,
            } in &package.type_origin_table
            {
                tx.execute(
                    "INSERT INTO package_type_origins(
                        package_id,
                        network,
                        module_name,
                        datatype_name,
                        origin)
                    VALUES($1, $2, $3, $4, $5)",
                    &[
                        &package.id.to_string(),
                        &client.network,
                        &module_name,
                        &datatype_name,
                        &origin.to_string(),
                    ],
                )
                .await?;
            }
        }
    }
    tx.commit().await?;
    Ok(data)
    /*
    };

    match inner.await {
        Ok(data) => Ok(data),
        Err(e) => {
            let rows = db.execute(
            "DELETE FROM download_object_tasks WHERE object_id = $1 AND network = $2 AND worker = $3",
            &[&object_id.to_string(), &client.network, &worker.to_string()],
        )
        .await?;
            assert!(rows == 1);
            return Err(e);
        }
    }*/
}

pub async fn get_or_download_object(mut params: DownloadObjectParams<'_>) -> Result<SuiObjectData> {
    let object = read_object_from_db(
        params.object_id,
        params.client.network.clone(),
        &mut params.db,
    )
    .await?;
    Ok(if let Some(package) = object {
        package
    } else {
        download_object(params).await?
    })
}
