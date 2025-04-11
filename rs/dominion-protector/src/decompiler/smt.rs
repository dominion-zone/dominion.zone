use std::fmt::{self, format};

use anyhow::Result;
use move_binary_format::{
    file_format::{SignatureToken, TypeSignature},
    CompiledModule,
};
use sui_types::base_types::ObjectID;

pub fn decompile_symbol(package_id: &ObjectID, module_name: &str, name: &str) -> String {
    format!("|{}::{}::{}|", package_id, module_name, name)
}

pub fn decompile_type_signature(module: &CompiledModule, s: &SignatureToken) -> String {
    use std::fmt::Write;
    match &s {
        SignatureToken::Bool => "Bool".to_string(),
        SignatureToken::U8 => "Int".to_string(),
        SignatureToken::U64 => "Int".to_string(),
        SignatureToken::U128 => "Int".to_string(),
        SignatureToken::Address => "Int".to_string(),
        SignatureToken::Signer => "Int".to_string(),
        SignatureToken::Vector(signature_token) => format!(
            "(Vector {})",
            decompile_type_signature(module, signature_token.as_ref())
        ),
        SignatureToken::Datatype(datatype_handle_index) => {
            let handle = module.datatype_handle_at(*datatype_handle_index);
            let m = module.module_handle_at(handle.module);
            decompile_symbol(
                &ObjectID::from_address(module.address_identifier_at(m.address).clone()),
                module.identifier_at(m.name).as_str(),
                module.identifier_at(handle.name).as_str(),
            )
        }
        SignatureToken::DatatypeInstantiation(i) => {
            let handle = module.datatype_handle_at(i.0);
            let m = module.module_handle_at(handle.module);
            let name = decompile_symbol(
                &ObjectID::from_address(module.address_identifier_at(m.address).clone()),
                module.identifier_at(m.name).as_str(),
                module.identifier_at(handle.name).as_str(),
            );
            let mut r = String::new();
            write!(&mut r, "({}", name).unwrap();
            for t in &i.1 {
                write!(&mut r, " {}", decompile_type_signature(module, t)).unwrap();
            }
            r
        },
        SignatureToken::Reference(signature_token) => todo!(),
        SignatureToken::MutableReference(signature_token) => todo!(),
        SignatureToken::TypeParameter(index) => format!("T{}", index),
        SignatureToken::U16 => "Int".to_string(),
        SignatureToken::U32 => "Int".to_string(),
        SignatureToken::U256 => "Int".to_string(),
    }
}

pub fn decompile_module<W: fmt::Write>(
    package_id: &ObjectID,
    module: &CompiledModule,
    mut w: W,
) -> Result<()> {
    if !module.struct_defs().is_empty() || !module.enum_defs().is_empty() {
        write!(w, "(declare-datatypes (")?;
        for s in module.struct_defs() {
            let handle = module.datatype_handle_at(s.struct_handle);
            let name = decompile_symbol(
                package_id,
                module.name().as_str(),
                module.identifier_at(handle.name).as_str(),
            );
            write!(w, "\n  ({} {})", name, handle.type_parameters.len())?;
        }
        for e in module.enum_defs() {
            let handle = module.datatype_handle_at(e.enum_handle);
            let name = decompile_symbol(
                package_id,
                module.name().as_str(),
                module.identifier_at(handle.name).as_str(),
            );
            write!(w, "\n  ({} {})", name, handle.type_parameters.len())?;
        }
        write!(w, ")\n  (")?;
        for s in module.struct_defs() {
            let handle = module.datatype_handle_at(s.struct_handle);
            let name = decompile_symbol(
                package_id,
                module.name().as_str(),
                module.identifier_at(handle.name).as_str(),
            );
            write!(w, "\n  ")?;
            if !handle.type_parameters.is_empty() {
                write!(w, "(par (")?;
                for i in 0..handle.type_parameters.len() {
                    if i > 0 {
                        write!(w, " ")?;
                    }
                    write!(w, "T{}", i)?;
                }
                write!(w, ") ")?;
            }
            write!(w, "(({}", name)?;

            for f in s.fields().unwrap_or_default() {
                write!(
                    w,
                    " ({} {})",
                    module.identifier_at(f.name).as_str(),
                    decompile_type_signature(&module, &f.signature.0),
                )?;
            }
            write!(w, "))")?;
            if !handle.type_parameters.is_empty() {
                write!(w, ")")?;
            }
        }
        for e in module.enum_defs() {
            /*
            let handle = module.datatype_handle_at(e.enum_handle);
            let name = module.identifier_at(handle.name).as_str();
            write!(w, "\n  ({} {})", name, handle.type_parameters.len())?;
            */
            todo!()
        }
        writeln!(w, "))")?;
    }
    Ok(())
}
