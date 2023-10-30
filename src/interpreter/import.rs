use std::fs;

use crate::{
    comfy::{fs::import_fs_fn, math::import_math_fn},
    parser::ast::import::{ImportSource, ImportSpecifier},
    script::ComfyScript,
};

use super::SymbolTable;

pub fn import(
    symbol_table: &mut SymbolTable,
    source: ImportSource,
    specifiers: Vec<ImportSpecifier>,
) -> Result<(), String> {
    match source.console_print().as_str() {
        "math" => {
            for specifier in specifiers {
                let f = import_math_fn(specifier.imported.name)?;
                symbol_table.functions.insert(specifier.local.name, f);
            }
        }
        "fs" => {
            for specifier in specifiers {
                let f = import_fs_fn(specifier.imported.name)?;
                symbol_table.functions.insert(specifier.local.name, f);
            }
        }
        "json" => todo!(),
        "thread" => todo!(),
        "time" => todo!(),
        "http" => todo!(),
        "env" => todo!(),
        "collections" => todo!(),
        "input_output" => todo!(),
        file_name => {
            let file_content = if file_name.ends_with(".cfs") {
                fs::read_to_string(file_name)
            } else {
                fs::read_to_string(file_name.to_owned() + ".cfs")
            };

            match file_content {
                Ok(content) => {
                    let script = ComfyScript::new(file_name, content);

                    match script.execute_as_import() {
                        Ok(mut importing_table) => {
                            for specifier in specifiers {
                                let exported_fn =
                                    importing_table.export_function(&specifier.local.name)?;

                                symbol_table
                                    .functions
                                    .insert(specifier.local.name, exported_fn);
                            }
                        }
                        Err((e, file)) => {
                            e.print(file).unwrap();
                        }
                    };
                }
                Err(_) => return Err(format!("Failed to find `{}` file", file_name)),
            }
        } // check for importing in another file
    }

    Ok(())
}
