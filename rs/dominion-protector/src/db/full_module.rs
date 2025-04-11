use std::collections::BTreeMap;

use anyhow::Result;
use move_binary_format::{file_format::Visibility, CompiledModule};
use sqlx::{Acquire, Postgres};
use sui_sdk::types::base_types::ObjectID;

use super::{function::Function, package_module::PackageModule, structure::Structure};

pub async fn save_module<'a, A>(
    db: A,
    package_id: ObjectID,
    network: &str,
    module_name: &str,
    module_bytecode: &[u8],
    type_origins: &BTreeMap<(String, String), ObjectID>,
) -> Result<()>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut db = db.acquire().await?;

    let compiled = CompiledModule::deserialize_with_defaults(module_bytecode)?;

    let module = PackageModule {
        package_id: package_id.to_string(),
        network: network.to_string(),
        module_name: module_name.to_string(),
        module_bytecode: module_bytecode.to_vec(),
    };

    module.save(&mut *db).await?;

    for struct_def in compiled.struct_defs() {
        let handle = compiled.datatype_handle_at(struct_def.struct_handle);
        let struct_name = compiled.identifier_at(handle.name).as_str();
        let origin = type_origins
            .get(&(module_name.to_string(), struct_name.to_string()))
            .unwrap();
        let db_structure = Structure {
            package_id: package_id.to_string(),
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

    for enum_def in compiled.enum_defs() {
        let handle = compiled.datatype_handle_at(enum_def.enum_handle);
        let enum_name = compiled.identifier_at(handle.name).as_str();
        let origin = type_origins
            .get(&(module_name.to_string(), enum_name.to_string()))
            .unwrap();

        let db_enum = Structure {
            package_id: package_id.to_string(),
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

    for function in compiled.function_defs() {
        let handle = compiled.function_handle_at(function.function);
        let params = compiled.signature_at(handle.parameters);
        let return_ = compiled.signature_at(handle.return_);
        let db_function = Function {
            package_id: package_id.to_string(),
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

    Ok(())
}
