use std::fs;

use crate::{
    comfy::{fs::import_fs_fn, math::import_math_fn, time::import_time_fn},
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
                let f = import_math_fn(specifier.imported.value())?;
                symbol_table
                    .functions
                    .insert(specifier.local.value(), f.into());
            }
        }
        "fs" => {
            for specifier in specifiers {
                let f = import_fs_fn(specifier.imported.value())?;
                symbol_table
                    .functions
                    .insert(specifier.local.value(), f.into());
            }
        }
        "json" => todo!(),
        "thread" => todo!(),
        "time" => {
            for specifier in specifiers {
                let f = import_time_fn(specifier.imported.value())?;
                symbol_table
                    .functions
                    .insert(specifier.local.value(), f.into());
            }
        }
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

                    match script.execute_as_import(&symbol_table) {
                        Ok(mut importing_table) => {
                            for specifier in specifiers {
                                let exported_fn =
                                    importing_table.export_function(&specifier.imported.value())?;

                                symbol_table
                                    .functions
                                    .insert(specifier.local.value(), exported_fn);
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
