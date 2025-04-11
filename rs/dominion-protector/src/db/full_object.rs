use std::{
    collections::{BTreeMap, HashMap},
    u64,
};

use anyhow::{bail, Context, Result};
use move_binary_format::{file_format::Visibility, CompiledModule};
use sqlx::{Acquire, Postgres};
use sui_sdk::{
    rpc_types::{SuiObjectData, SuiRawData, SuiRawMovePackage},
    types::{
        base_types::{ObjectID, SequenceNumber},
        move_package::{TypeOrigin, UpgradeInfo},
    },
};

use super::{
    full_module::save_module, function::Function, object::Object, package_linkage::PackageLinkage,
    package_module::PackageModule, structure::Structure,
};
use sui_types::object::Data;

pub async fn load_object<'a, A>(
    db: A,
    network: &str,
    object_id: &ObjectID,
) -> Result<Option<SuiObjectData>>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut db = db.acquire().await?;
    let db_object = Object::load(&mut *db, object_id, network).await?;
    let db_object = match db_object {
        Some(db_object) => db_object,
        None => return Ok(None),
    };

    let mut object: SuiObjectData = (&db_object).try_into()?;

    if object.type_.as_ref().unwrap().is_package() {
        let modules = PackageModule::load_all_by_package(&mut *db, object_id, network).await?;
        let structures =
            Structure::load_all_by_package(&mut *db, &object.object_id.to_string(), network)
                .await?;
        let linkages = PackageLinkage::load_all_by_package(&mut *db, object_id, network).await?;
        let mut module_map = BTreeMap::new();
        let type_origin_table = structures
            .into_iter()
            .map(|structure| {
                Ok(TypeOrigin {
                    module_name: structure.module_name,
                    datatype_name: structure.datatype_name,
                    package: ObjectID::from_hex_literal(&structure.origin)?,
                })
            })
            .collect::<Result<Vec<_>>>()?;
        for module in modules {
            module_map.insert(module.module_name.clone(), module.module_bytecode.clone());
        }
        object.bcs = Some(SuiRawData::Package(SuiRawMovePackage {
            id: object_id.clone(),
            version: SequenceNumber::from_u64(u64::try_from(db_object.version)?),
            module_map,
            type_origin_table,
            linkage_table: linkages
                .into_iter()
                .map(|linkage| {
                    Ok((
                        ObjectID::from_hex_literal(&linkage.dependency_id)?,
                        UpgradeInfo {
                            upgraded_id: ObjectID::from_hex_literal(&linkage.upgraded_id)?,
                            upgraded_version: SequenceNumber::from_u64(u64::try_from(
                                linkage.upgraded_version,
                            )?),
                        },
                    ))
                })
                .collect::<Result<BTreeMap<_, _>>>()?,
        }));
    }
    Ok(Some(object))
}

pub async fn save_object<'a, A>(db: A, network: &str, object: &Data) -> Result<()>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut db = db.acquire().await?;
    let db_object: Object = (object, network).try_into()?;
    db_object.save(&mut *db).await?;
    match object {
        Data::Package(package) => {
            let type_origins = package.type_origin_map();
            for (module_name, module_bytecode) in package.serialized_module_map() {
                save_module(
                    &mut *db,
                    object.id(),
                    network,
                    &module_name,
                    &module_bytecode,
                    &type_origins,
                )
                .await?;
            }

            for (dependency_id, upgrade_info) in package.linkage_table() {
                let linkage = PackageLinkage {
                    package_id: object.id().to_string(),
                    network: network.to_string(),
                    dependency_id: dependency_id.to_string(),
                    upgraded_id: upgrade_info.upgraded_id.to_string(),
                    upgraded_version: upgrade_info.upgraded_version.value().try_into()?,
                };
                linkage.save(&mut *db).await?;
            }
        }
        Data::Move(_) => {
            bail!("MoveObject not supported yet");
        }
    }
    Ok(())
}

pub async fn save_rpc_object<'a, A>(db: A, network: &str, object: &SuiObjectData) -> Result<()>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut db = db.acquire().await?;

    let db_object: Object = (object, network).try_into()?;
    db_object.save(&mut *db).await?;

    match object.bcs.as_ref().unwrap() {
        SuiRawData::MoveObject(_) => {
            bail!("MoveObject not supported yet");
        }
        SuiRawData::Package(package) => {
            let package = package.to_move_package(u64::MAX)?;
            let type_origins = package.type_origin_map();
            for (module_name, module_bytecode) in package.serialized_module_map() {
                save_module(
                    &mut *db,
                    object.object_id,
                    network,
                    &module_name,
                    &module_bytecode,
                    &type_origins,
                )
                .await?;
            }

            for (dependency_id, upgrade_info) in package.linkage_table() {
                let linkage = PackageLinkage {
                    package_id: object.object_id.to_string(),
                    network: network.to_string(),
                    dependency_id: dependency_id.to_string(),
                    upgraded_id: upgrade_info.upgraded_id.to_string(),
                    upgraded_version: upgrade_info.upgraded_version.value().try_into()?,
                };
                linkage.save(&mut *db).await?;
            }
        }
    }

    Ok(())
}
