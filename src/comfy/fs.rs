// handling filesystem, paths and etct

use std::{fs, rc::Rc};

use crate::{
    interpreter::{InterpretedFn, SymbolTable},
    parser::ast::{literal_value::LiteralValue, Expression},
};

use super::{expected_string_arg, expected_x_args};

pub fn import_fs_fn(name: String) -> Result<InterpretedFn, String> {
    let result = match name.as_str() {
        "read_to_string" => InterpretedFn {
            name: name.to_owned(),
            executable: Rc::new(read_to_string()),
        },
        "rename" => InterpretedFn {
            name: name.to_owned(),
            executable: Rc::new(rename()),
        },
        "write" => InterpretedFn {
            name: name.to_owned(),
            executable: Rc::new(write()),
        },
        _ => return Err(format!("'fs' package does not export a `{}` member", name)),
    };

    Ok(result)
}

fn read_to_string() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        expected_x_args("read_to_string", 1, &args)?;

        let file_path: String =
            expected_string_arg(symbol_table, "read_to_string", args[0].to_owned())?.into();

        match fs::read_to_string(&file_path) {
            Ok(content) => Ok(Expression::Ok(Box::new(content.into()))),
            Err(_) => Ok(Expression::Err(format!(
                "Cannot read `{}`. File does not exist",
                file_path
            ))),
        }
    }
}

fn rename() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        expected_x_args("rename", 2, &args)?;

        let file_path: String =
            expected_string_arg(symbol_table, "rename", args[0].to_owned())?.into();
        let new_name: String =
            expected_string_arg(symbol_table, "rename", args[1].to_owned())?.into();

        match fs::rename(&file_path, new_name) {
            Ok(_) => Ok(Expression::Literal {
                value: LiteralValue::Nil,
                raw: "nil".to_owned(),
            }),
            Err(_) => Ok(Expression::Err(format!(
                "Cannot rename `{}`. File does not exist",
                file_path
            ))),
        }
    }
}

fn write() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        expected_x_args("write", 2, &args)?;

        let file_path: String =
            expected_string_arg(symbol_table, "write", args[0].to_owned())?.into();
        let content: String =
            expected_string_arg(symbol_table, "write", args[1].to_owned())?.into();

        match fs::write(&file_path, content) {
            Ok(_) => Ok(Expression::Literal {
                value: LiteralValue::Nil,
                raw: "nil".to_owned(),
            }),
            Err(e) => Ok(Expression::Err(e.to_string())),
        }
    }
}
