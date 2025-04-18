use anyhow::Result;
use move_binary_format::CompiledModule;
use move_bytecode_source_map::mapping::SourceMapping;
use move_disassembler::disassembler::{Disassembler, DisassemblerOptions};
use move_ir_types::location::Spanned;
use sqlx::{Acquire, Postgres};
use sui_types::base_types::ObjectID;

use crate::db::sources::ModuleSource;

pub mod revela;
pub mod smt;

pub async fn decompile_module_to_smt<'a, A>(
    db: A,
    network: &str,
    package_id: ObjectID,
    module: &CompiledModule,
) -> Result<ModuleSource> {
    // let mut db = db.acquire().await?;

    let mut smt = String::new();
    smt::decompile_module(&package_id, module, &mut smt)?;
    println!("{}", &smt);

    let sources = ModuleSource {
        package_id: package_id.to_string(),
        module_name: module.name().to_string(),
        network: network.to_string(),
        source: smt,
        kind: "smt".to_string(),
    };
    // sources.save(&mut *db).await?;
    Ok(sources)
}

pub async fn decompile_module_with_disasm<'a, A>(
    db: A,
    network: &str,
    package_id: ObjectID,
    module: &CompiledModule,
) -> Result<ModuleSource>
where
    A: Acquire<'a, Database = Postgres>,
{
    let mut disassembler_options = DisassemblerOptions::new();
    disassembler_options.print_code = true;
    disassembler_options.only_externally_visible = false;
    disassembler_options.print_basic_blocks = true;
    disassembler_options.print_locals = true;

    // TODO: make source mapping work with the Move source language
    let no_loc = Spanned::unsafe_no_loc(()).loc;

    let source_mapping = SourceMapping::new_without_source_map(&module, no_loc)?;

    let disassembler = Disassembler::new(source_mapping, disassembler_options);

    let dissassemble_string = disassembler.disassemble()?;

    if let Some(i) = dissassemble_string.find("\0") {
        println!(
            "Found null character at index: {}: {}",
            i,
            dissassemble_string[i.saturating_sub(10)..i + 10].to_string()
        );
    }

    // println!("sources: {}", &dissassemble_string);

    let sources = ModuleSource {
        package_id: package_id.to_string(),
        module_name: module.name().to_string(),
        network: network.to_string(),
        source: dissassemble_string,
        kind: "disassembled".to_string(),
    };
    let mut db = db.acquire().await?;
    sources.save(&mut *db).await?;
    Ok(sources)
}
