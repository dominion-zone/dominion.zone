use std::{collections::BTreeMap, str::FromStr};

use sui_sdk::{
    rpc_types::{SuiObjectData, SuiRawData, SuiRawMovePackage},
    types::{
        base_types::{ObjectID, ObjectType, SequenceNumber, SuiAddress},
        digests::ObjectDigest,
        move_package::{TypeOrigin, UpgradeInfo},
        object::Owner,
    },
};
use tokio_postgres::{
    types::{FromSql, ToSql},
    Client,
};

use anyhow::Result;

#[derive(Debug, ToSql, FromSql)]
#[postgres(name = "owner_type")]
pub enum OwnerType {
    AddressOwner,
    ObjectOwner,
    Shared,
    Immutable,
    ConsensusV2,
}

pub async fn create_objects_tables_if_needed(client: &Client) -> Result<()> {
    // CREATE TYPE owner_type AS ENUM ('AddressOwner', 'ObjectOwner', 'Shared', 'Immutable', 'ConsensusV2');
    /*
       CREATE TABLE IF NOT EXISTS download_object_tasks (
           object_id       CHAR(66) NOT NULL,
           network         VARCHAR(10) NOT NULL,
           worker          TEXT,
           updated_at      TIMESTAMPTZ NOT NULL DEFAULT Now(),
           PRIMARY KEY(object_id, network)
       );
    */
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS objects (
            id              CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            version         BIGINT NOT NULL,
            digest          VARCHAR(64) NOT NULL,
            type            TEXT NOT NULL,
            owner_type      owner_type NOT NULL,
            owner           VARCHAR(66),
            initial_shared_version BIGINT,
            read_at         TIMESTAMPTZ NOT NULL DEFAULT Now(),
            PRIMARY KEY(id, network)
        );
        CREATE TABLE IF NOT EXISTS package_modules (
            package_id      CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            name            TEXT NOT NULL,
            bytecode        BYTEA NOT NULL,
            PRIMARY KEY(package_id, network, name),
            FOREIGN KEY(package_id, network) REFERENCES objects(id, network) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS package_type_origins (
            package_id      CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            module_name     TEXT NOT NULL,
            datatype_name   TEXT NOT NULL,
            origin          CHAR(66) NOT NULL,
            PRIMARY KEY(package_id, network, module_name, datatype_name),
            FOREIGN KEY(package_id, network) REFERENCES objects(id, network) ON DELETE CASCADE,
            FOREIGN KEY(package_id, network, module_name) REFERENCES package_modules(package_id, network, name) ON DELETE CASCADE
        );
        CREATE TABLE IF NOT EXISTS package_linkage (
            package_id      CHAR(66) NOT NULL,
            network         VARCHAR(10) NOT NULL,
            dependency_id   CHAR(66) NOT NULL,
            upgraded_id     CHAR(66) NOT NULL,
            upgraded_version BIGINT NOT NULL,
            PRIMARY KEY(package_id, network, dependency_id),
            FOREIGN KEY(package_id, network) REFERENCES objects(id, network) ON DELETE CASCADE,
        );
    ").await?;
    Ok(())
}

pub async fn read_object_from_db(
    object_id: ObjectID,
    network: String,
    db: &mut Client,
) -> Result<Option<SuiObjectData>> {
    let rows = db
        .query(
            "SELECT
        version,
        digest,
        type,
        owner_type,
        owner,
        initial_shared_version
    FROM objects WHERE id = $1 AND network = $2",
            &[&object_id.to_string(), &network],
        )
        .await?;
    if rows.is_empty() {
        return Ok(None);
    }
    let row = &rows[0];
    let version = SequenceNumber::from(u64::try_from(row.get::<usize, i64>(0))?);
    let bcs = if row.get::<usize, &str>(2) == "package" {
        let modules = db
            .query(
                "SELECT name, bytecode FROM package_modules WHERE package_id = $1 AND network = $2",
                &[&object_id.to_string(), &network],
            )
            .await?;
        let type_origins = db
            .query(
                "SELECT module_name, datatype_name, origin FROM package_type_origins WHERE package_id = $1 AND network = $2",
                &[&object_id.to_string(), &network],
            )
            .await?;
        let linkage_table = db
            .query(
                "SELECT
            network,
            dependency_id,
            upgraded_id,
            upgraded_version
        FROM package_linkage_table WHERE package_id = $1 AND network = $2",
                &[&object_id.to_string(), &network],
            )
            .await?
            .iter()
            .map(|row| {
                Ok::<(ObjectID, UpgradeInfo), anyhow::Error>((
                    ObjectID::from_hex_literal(row.get(1))?,
                    UpgradeInfo {
                        upgraded_id: ObjectID::from_str(row.get(2)).unwrap(),
                        upgraded_version: SequenceNumber::from(u64::try_from(
                            row.get::<usize, i64>(3),
                        )?),
                    },
                ))
            })
            .collect::<Result<BTreeMap<ObjectID, UpgradeInfo>>>()?;
        Some(SuiRawData::Package(SuiRawMovePackage {
            id: object_id,
            version,
            module_map: modules
                .into_iter()
                .map(|row| (row.get(0), row.get(1)))
                .collect(),
            type_origin_table: type_origins
                .into_iter()
                .map(|row| TypeOrigin {
                    module_name: row.get(0),
                    datatype_name: row.get(1),
                    package: ObjectID::from_str(row.get(2)).unwrap(),
                })
                .collect(),
            linkage_table,
        }))
    } else {
        None
    };

    Ok(Some(SuiObjectData {
        object_id,
        version,
        digest: ObjectDigest::from_str(row.get(1))?,
        type_: Some(ObjectType::from_str(row.get(2))?),
        owner: Some(match row.get(3) {
            OwnerType::AddressOwner => Owner::AddressOwner(SuiAddress::from_str(row.get(4))?),
            OwnerType::ObjectOwner => Owner::ObjectOwner(SuiAddress::from_str(row.get(4))?),
            OwnerType::Shared => Owner::Shared {
                initial_shared_version: SequenceNumber::from(u64::try_from(
                    row.get::<usize, i64>(5),
                )?),
            },
            OwnerType::Immutable => Owner::Immutable,
            OwnerType::ConsensusV2 => todo!(),
        }),
        bcs,
        previous_transaction: None,
        storage_rebate: None,
        display: None,
        content: None,
    }))
}
