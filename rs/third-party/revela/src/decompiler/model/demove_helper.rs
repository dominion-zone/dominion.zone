use std::{borrow::Borrow, collections::BTreeMap, rc::Rc};

use codespan::Files;
use itertools::Itertools;
use move_binary_format::CompiledModule;
use move_bytecode_source_map::source_map::SourceMap;
use move_command_line_common::files::FileHash;
use move_compiler::expansion::ast::Program;

use move_model::{
    ast::*,
    builder::{model_builder::ModelBuilder, module_builder::ModuleBuilder},
    model::*,
    symbol::*,
};

#[allow(dead_code)]
pub fn dummy_module_data(name: ModuleName, id: usize) -> ModuleData {
    ModuleData::stub(name, ModuleId::new(id), Default::default())
}

pub fn run_stackless_compiler(env: &mut GlobalEnv, program: Program) {
    env.add_source(FileHash::empty(), Rc::new(BTreeMap::new()), "", "", false);
    (env.file_hash_map).insert(
        FileHash::empty(),
        (
            "".to_string(),
            Files::<String>::default().add("".to_string(), "".to_string()),
        ),
    );

    let mut builder: ModelBuilder<'_> = ModelBuilder::new(env);

    let modules = units
        .into_iter()
        .flat_map(|unit| {
            let module_ident = unit.module_ident();
            let expanded_module = match eprog.modules.remove(&module_ident) {
                Some(m) => m,
                None => {
                    warn!(
                        "[internal] cannot associate bytecode module `{}` with AST",
                        module_ident
                    );
                    return None;
                }
            };
            Some((
                module_ident,
                expanded_module,
                unit.named_module.module,
                unit.named_module.source_map,
            ))
        })
        .enumerate();
    for (module_count, (module_id, expanded_module, compiled_module, source_map)) in modules {
        let loc = builder.to_loc(&expanded_module.loc);
        let addr_bytes = builder.resolve_address(&loc, &module_id.value.address);
        let module_name = ModuleName::from_address_bytes_and_name(
            addr_bytes,
            builder
                .env
                .symbol_pool()
                .make(&module_id.value.module.0.value),
        );
        let module_id = ModuleId::new(module_count);
        let mut module_translator = ModuleBuilder::new(&mut builder, module_id, module_name);
        module_translator.translate(loc, expanded_module, compiled_module, source_map);
    }

    /*
    for module in env.module_data.iter_mut() {
        for fun_data in module.function_data.values_mut() {
            *fun_data.called_funs.borrow_mut() = Some(
                fun_data
                    .def
                    .borrow()
                    .as_ref()
                    .map(|e| e.called_funs())
                    .unwrap_or_default(),
            )
        }
    }*/
}
