use std::collections::{BTreeMap, HashMap};

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
    function::Function, object::Object, package_linkage::PackageLinkage,
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
            for (name, bytecode) in package.serialized_module_map() {
                let module = PackageModule {
                    package_id: object.id().to_string(),
                    network: network.to_string(),
                    module_name: name.to_string(),
                    module_bytecode: bytecode.clone(),
                };
                module.save(&mut *db).await?;
            }
            let modules = package
                .serialized_module_map()
                .iter()
                .map(|(name, bytecode)| {
                    Ok((
                        name.clone(),
                        CompiledModule::deserialize_with_defaults(bytecode)?,
                    ))
                })
                .collect::<Result<HashMap<_, _>>>()?;
            /*

            for TypeOrigin {
                module_name,
                // `struct_name` alias to support backwards compatibility with the old name
                datatype_name,
                package: origin,
            } in package.type_origin_table()
            {
                let module = modules.get(module_name).unwrap();
                let structure = module
                    .struct_defs()
                    .iter()
                    .find(|def| {
                        let handle = module.datatype_handle_at(def.struct_handle);
                        let struct_name = module.identifier_at(handle.name).as_str();
                        struct_name == datatype_name.as_str()
                    });
                let handle = module.datatype_handle_at(structure.struct_handle);
                let db_structure = Structure {
                    package_id: object.id().to_string(),
                    network: network.to_string(),
                    module_name: module_name.to_string(),
                    datatype_name: datatype_name.to_string(),
                    origin: origin.to_string(),
                    field_count: structure.declared_field_count()? as i32,
                    type_argument_count: handle.type_parameters.len() as i32,
                    source_code: None,
                    has_key: handle.abilities.has_key(),
                    has_copy: handle.abilities.has_copy(),
                    has_drop: handle.abilities.has_drop(),
                    has_store: handle.abilities.has_store(),
                };
                db_structure.save(&mut *db).await?;
            }
            */
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
            let type_origins = package.type_origin_map();
            for (module_name, module) in modules {
                for struct_def in module.struct_defs() {
                    let handle = module.datatype_handle_at(struct_def.struct_handle);
                    let struct_name = module.identifier_at(handle.name).as_str();
                    let origin = type_origins
                        .get(&(module_name.clone(), struct_name.to_string()))
                        .unwrap();
                    let db_structure = Structure {
                        package_id: object.id().to_string(),
                        network: network.to_string(),
                        module_name: module_name.to_string(),
                        datatype_name: struct_name.to_string(),
                        origin: origin.to_string(),
                        field_count: struct_def.declared_field_count()? as i32,
                        type_argument_count: handle.type_parameters.len() as i32,
                        source_code: None,
                        has_key: handle.abilities.has_key(),
                        has_copy: handle.abilities.has_copy(),
                        has_drop: handle.abilities.has_drop(),
                        has_store: handle.abilities.has_store(),
                    };
                    db_structure.save(&mut *db).await?;
                }
                for enum_def in module.enum_defs() {
                    let handle = module.datatype_handle_at(enum_def.enum_handle);
                    let enum_name = module.identifier_at(handle.name).as_str();
                    let origin = type_origins
                        .get(&(module_name.clone(), enum_name.to_string()))
                        .unwrap();

                    let db_enum = Structure {
                        package_id: object.id().to_string(),
                        network: network.to_string(),
                        module_name: module_name.to_string(),
                        datatype_name: enum_name.to_string(),
                        origin: origin.to_string(),
                        field_count: -(enum_def.variants.len() as i32),
                        type_argument_count: handle.type_parameters.len() as i32,
                        source_code: None,
                        has_key: handle.abilities.has_key(),
                        has_copy: handle.abilities.has_copy(),
                        has_drop: handle.abilities.has_drop(),
                        has_store: handle.abilities.has_store(),
                    };
                    db_enum.save(&mut *db).await?;
                }
                for function in module.function_defs() {
                    let handle = module.function_handle_at(function.function);
                    let params = module.signature_at(handle.parameters);
                    let return_ = module.signature_at(handle.return_);
                    let db_function = Function {
                        package_id: object.id().to_string(),
                        network: network.to_string(),
                        module_name: module_name.to_string(),
                        function_name: handle.name.to_string(),
                        visibility: match function.visibility {
                            Visibility::Private => super::function::Visibility::Private,
                            Visibility::Public => super::function::Visibility::Public,
                            Visibility::Friend => super::function::Visibility::Friend,
                        },
                        is_entry: function.is_entry,
                        is_initializer: &handle.name.to_string() == "init",
                        type_argument_count: handle.type_parameters.len() as i32,
                        parameter_count: params.len() as i32,
                        return_count: return_.len() as i32,
                        source_code: None,
                    };
                    db_function.save(&mut *db).await?;
                }
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
            for (name, bytecode) in &package.module_map {
                let module = PackageModule {
                    package_id: object.object_id.to_string(),
                    network: network.to_string(),
                    module_name: name.to_string(),
                    module_bytecode: bytecode.clone(),
                };
                module.save(&mut *db).await?;
            }
            let modules = package
                .module_map
                .iter()
                .map(|(name, bytecode)| {
                    Ok((
                        name.clone(),
                        CompiledModule::deserialize_with_defaults(bytecode)?,
                    ))
                })
                .collect::<Result<HashMap<_, _>>>()?;
            for TypeOrigin {
                module_name,
                // `struct_name` alias to support backwards compatibility with the old name
                datatype_name,
                package: origin,
            } in &package.type_origin_table
            {
                let module = modules.get(module_name).unwrap();
                let structure = module
                    .struct_defs()
                    .iter()
                    .find(|def| {
                        let handle = module.datatype_handle_at(def.struct_handle);
                        let struct_name = module.identifier_at(handle.name).as_str();
                        struct_name == datatype_name.as_str()
                    })
                    .unwrap();
                let handle = module.datatype_handle_at(structure.struct_handle);
                let db_structure = Structure {
                    package_id: object.object_id.to_string(),
                    network: network.to_string(),
                    module_name: module_name.to_string(),
                    datatype_name: datatype_name.to_string(),
                    origin: origin.to_string(),
                    field_count: structure.declared_field_count()? as i32,
                    type_argument_count: handle.type_parameters.len() as i32,
                    source_code: None,
                    has_key: handle.abilities.has_key(),
                    has_copy: handle.abilities.has_copy(),
                    has_drop: handle.abilities.has_drop(),
                    has_store: handle.abilities.has_store(),
                };
                db_structure.save(&mut *db).await?;
            }
            for (dependency_id, upgrade_info) in &package.linkage_table {
                let linkage = PackageLinkage {
                    package_id: object.object_id.to_string(),
                    network: network.to_string(),
                    dependency_id: dependency_id.to_string(),
                    upgraded_id: upgrade_info.upgraded_id.to_string(),
                    upgraded_version: upgrade_info.upgraded_version.value().try_into()?,
                };
                linkage.save(&mut *db).await?;
            }
            for (module_name, module) in modules {
                for function in module.function_defs() {
                    let handle = module.function_handle_at(function.function);
                    let params = module.signature_at(handle.parameters);
                    let return_ = module.signature_at(handle.return_);
                    let db_function = Function {
                        package_id: object.object_id.to_string(),
                        network: network.to_string(),
                        module_name: module_name.to_string(),
                        function_name: handle.name.to_string(),
                        visibility: match function.visibility {
                            Visibility::Private => super::function::Visibility::Private,
                            Visibility::Public => super::function::Visibility::Public,
                            Visibility::Friend => super::function::Visibility::Friend,
                        },
                        is_entry: function.is_entry,
                        is_initializer: &handle.name.to_string() == "init",
                        type_argument_count: handle.type_parameters.len() as i32,
                        parameter_count: params.len() as i32,
                        return_count: return_.len() as i32,
                        source_code: None,
                    };
                    db_function.save(&mut *db).await?;
                }
            }
        }
    }

    Ok(())
}
