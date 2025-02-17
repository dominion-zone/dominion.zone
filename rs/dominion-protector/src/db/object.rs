use std::{
    collections::{BTreeMap, HashMap},
    str::FromStr,
};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sui_sdk::{
    rpc_types::SuiObjectData,
    types::{
        base_types::{ObjectID, ObjectType, SequenceNumber, SuiAddress},
        digests::ObjectDigest,
        object::Owner,
    },
};

use anyhow::{Context, Result};
use sqlx::{query_as_unchecked, query_unchecked, Executor};
use sqlx::{FromRow, Postgres};

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, sqlx::Type,
)]
pub enum OwnerType {
    AddressOwner,
    ObjectOwner,
    Shared,
    Immutable,
    ConsensusV2,
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Object {
    pub object_id: String,                   // CHAR(66)
    pub network: String,                     // VARCHAR(10)
    pub version: i64,                        // BIGINT
    pub digest: String,                      // VARCHAR(64)
    pub object_type: String,                 // TEXT
    pub owner_type: OwnerType,               // ENUM OwnerType
    pub owner: Option<String>,               // VARCHAR(66), nullable
    pub initial_shared_version: Option<i64>, // BIGINT, nullable
    pub read_at: DateTime<Utc>,              // TIMESTAMPTZ, default Now()
}

impl Object {
    pub async fn load<'c, E: Executor<'c, Database = Postgres>>(
        db: E,
        object_id: &ObjectID,
        network: &str,
    ) -> Result<Option<Self>> {
        Ok(query_as_unchecked!(
            Object,
            "SELECT * FROM objects
            WHERE object_id = $1 AND network = $2",
            &object_id.to_string(),
            &network
        )
        .fetch_optional(db)
        .await?)
    }

    pub async fn save<'c, E: Executor<'c, Database = Postgres>>(&self, db: E) -> Result<()> {
        query_unchecked!(
            "INSERT INTO objects
            (object_id, network, version, digest, object_type, owner_type, owner, initial_shared_version)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (object_id, network) DO UPDATE
            SET version = $3, digest = $4, object_type = $5, owner_type = $6, owner = $7, initial_shared_version = $8",
            &self.object_id,
            &self.network,
            &self.version,
            &self.digest,
            &self.object_type,
            &self.owner_type,
            &self.owner,
            &self.initial_shared_version)
            .execute(db).await?;
        Ok(())
    }
}

impl TryFrom<&Object> for SuiObjectData {
    type Error = anyhow::Error;

    fn try_from(object: &Object) -> Result<Self> {
        let object_id = ObjectID::from_hex_literal(&object.object_id)?;
        let version = SequenceNumber::from(u64::try_from(object.version)?);
        let digest = ObjectDigest::from_str(&object.digest)?;
        let type_ = ObjectType::from_str(&object.object_type)?;
        let owner = match object.owner_type {
            OwnerType::AddressOwner => Owner::AddressOwner(SuiAddress::from_str(
                &object.owner.as_ref().context("owner value is required")?,
            )?),
            OwnerType::ObjectOwner => Owner::ObjectOwner(SuiAddress::from_str(
                &object.owner.as_ref().context("owner value is required")?,
            )?),
            OwnerType::Shared => Owner::Shared {
                initial_shared_version: SequenceNumber::from(u64::try_from(
                    object.initial_shared_version.unwrap(),
                )?),
            },
            OwnerType::Immutable => Owner::Immutable,
            OwnerType::ConsensusV2 => todo!(),
        };
        Ok(SuiObjectData {
            object_id,
            version,
            digest,
            type_: Some(type_),
            owner: Some(owner),
            bcs: None,
            previous_transaction: None,
            storage_rebate: None,
            display: None,
            content: None,
        })
    }
}

impl TryFrom<Object> for SuiObjectData {
    type Error = anyhow::Error;

    fn try_from(value: Object) -> Result<Self> {
        value.try_into()
    }
}

impl From<(&SuiObjectData, &str)> for Object {
    fn from((data, network): (&SuiObjectData, &str)) -> Self {
        let object_id = data.object_id.to_string();
        let version = i64::try_from(data.version.value()).unwrap();
        let digest = data.digest.to_string();
        let object_type = data.type_.as_ref().unwrap().to_string();
        let (owner_type, owner, initial_shared_version) = match data.owner.as_ref().unwrap() {
            Owner::AddressOwner(sui_address) => {
                (OwnerType::AddressOwner, Some(sui_address.to_string()), None)
            }
            Owner::ObjectOwner(sui_address) => {
                (OwnerType::ObjectOwner, Some(sui_address.to_string()), None)
            }
            Owner::Shared {
                initial_shared_version,
            } => (
                OwnerType::Shared,
                None,
                Some(i64::try_from(initial_shared_version.value()).unwrap()),
            ),
            Owner::Immutable => (OwnerType::Immutable, None, None),
            Owner::ConsensusV2 { .. } => (OwnerType::ConsensusV2, None, None),
        };
        let read_at = Utc::now();
        Self {
            object_id,
            network: network.to_owned(),
            version,
            digest,
            object_type,
            owner_type,
            owner,
            initial_shared_version,
            read_at,
        }
    }
}
/*
pub async fn read_object_from_db(
    object_id: ObjectID,
    network: String,
    db: &Db,
) -> Result<Option<SuiObjectData>> {
    let query = sqlx::query(
        "SELECT * FROM objects
            LEFT JOIN package_modules
                ON objects.object_id = package_modules.package_id
                AND objects.network = package_modules.network
            LEFT JOIN package_type_origins
                ON objects.object_id = package_type_origins.package_id
                AND objects.network = package_type_origins.network
                AND package_modules.module_name = package_type_origins.module_name
            LEFT JOIN package_linkage
                ON objects.object_id = package_linkage.package_id
                AND objects.network = package_linkage.network
            WHERE objects.object_id = $1 AND objects.network = $2",
    )
    .bind(object_id.to_string())
    .bind(network);
    let rows = query.fetch_all(&db.pool).await?;
    if rows.is_empty() {
        return Ok(None);
    }
    let row = &rows[0];
    let version = SequenceNumber::from(u64::try_from(row.get::<i64, &str>("version"))?);
    let type_: String = row.get("type");
    let bcs = if type_ == "package" {
        let mut modules = BTreeMap::<String, Vec<u8>>::new();
        for row in &rows {
            let module_name: Option<String> = row.get("module_name");
            if let Some(module_name) = module_name {
                modules
                    .entry(module_name)
                    .or_insert(row.get("module_bytecode"));
            }
        }
        let mut type_origins = BTreeMap::<(String, String), TypeOrigin>::new();
        for row in &rows {
            let module_name: Option<String> = row.get("module_name");
            if let Some(module_name) = module_name {
                let datatype_name: String = row.get("datatype_name");
                type_origins
                    .entry((module_name.clone(), datatype_name.clone()))
                    .or_insert(TypeOrigin {
                        module_name,
                        datatype_name,
                        package: ObjectID::from_hex_literal(row.get("origin"))?,
                    });
            }
        }
        let mut linkage_table = BTreeMap::<ObjectID, UpgradeInfo>::new();
        for row in &rows {
            let dependency_id: Option<String> = row.get("dependency_id");
            if let Some(dependency_id) = dependency_id {
                let dependency_id = ObjectID::from_hex_literal(&dependency_id)?;
                linkage_table.entry(dependency_id).or_insert(UpgradeInfo {
                    upgraded_id: ObjectID::from_hex_literal(row.get("upgraded_id"))?,
                    upgraded_version: SequenceNumber::from(u64::try_from(
                        row.get::<i64, &str>("upgraded_version"),
                    )?),
                });
            }
        }
        Some(SuiRawData::Package(SuiRawMovePackage {
            id: object_id,
            version,
            module_map: modules,
            type_origin_table: type_origins.into_values().collect(),
            linkage_table,
        }))
    } else {
        None
    };

    Ok(Some(SuiObjectData {
        object_id,
        version,
        digest: ObjectDigest::from_str(row.get("digest"))?,
        type_: Some(ObjectType::from_str(&type_)?),
        owner: Some(match row.get("owner_type") {
            OwnerType::AddressOwner => Owner::AddressOwner(SuiAddress::from_str(row.get("owner"))?),
            OwnerType::ObjectOwner => Owner::ObjectOwner(SuiAddress::from_str(row.get("owner"))?),
            OwnerType::Shared => Owner::Shared {
                initial_shared_version: SequenceNumber::from(u64::try_from(
                    row.get::<i64, &str>("initial_shared_version"),
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
*/
