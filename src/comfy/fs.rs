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
            executable: Rc::new(read_to_string(name)),
        },
        "rename" => InterpretedFn {
            name: name.to_owned(),
            executable: Rc::new(rename(name)),
        },
        _ => return Err(format!("'fs' package does not export a `{}` member", name)),
    };

    Ok(result)
}

fn read_to_string(
    value: String,
) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        expected_x_args(&value, 1, &args)?;

        expected_string_arg(&value, &args[0])?;
        let file_path: String = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        match fs::read_to_string(file_path) {
            Ok(content) => Ok(Expression::Ok(Box::new(content.into()))),
            Err(e) => Ok(Expression::Err(e.to_string())),
        }
    }
}
fn rename(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        expected_x_args(&value, 2, &args)?;

        expected_string_arg(&value, &args[0])?;
        expected_string_arg(&value, &args[1])?;
        let file_path: String = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let new_name: String = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        match fs::rename(file_path, new_name) {
            Ok(_) => Ok(Expression::Literal {
                value: LiteralValue::Nil,
                raw: "nil".to_owned(),
            }),
            Err(e) => Ok(Expression::Err(e.to_string())),
        }
    }
}
