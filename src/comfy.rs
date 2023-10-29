// standard library
mod collections;
mod env;
mod fs;
mod http;
mod input_output;
mod json;
pub mod math;
mod thread;
mod time;

use std::io::{self, Write};

use crate::{
    interpreter::SymbolTable,
    parser::ast::{literal_value::LiteralValue, Expression},
};

pub fn print(symbol_table: &SymbolTable, args: Vec<Expression>) -> Result<Expression, String> {
    if args.len() != 1 {
        return Err("Expected 1 argument for function `print`".into());
    }

    println!("{}", symbol_table.evaluate_expr(args[0].to_owned())?);

    Ok(Expression::Literal {
        value: crate::parser::ast::literal_value::LiteralValue::Nil,
        raw: "nil".to_owned(),
    })
}

pub fn input(symbol_table: &SymbolTable, args: Vec<Expression>) -> Result<Expression, String> {
    if args.len() < 1 || args.len() > 2 {
        return Err("Expected exactly 1 or 2 arguments for function `input`".into());
    }

    let prompt = symbol_table
        .evaluate_expr(args[0].to_owned())?
        .console_print();

    print!("{}", prompt);
    let _ = io::stdout().flush();

    let mut input = String::new();
    if let Err(_) = io::stdin().read_line(&mut input) {
        return Err("Failed to read line".to_owned());
    }

    if args.len() == 2 {
        let restrain_empty = match symbol_table.evaluate_expr(args[1].to_owned())? {
            Expression::Literal { value, .. } => match value {
                LiteralValue::Boolean(b) => b,
                _ => return Err("Second argument expected to be a boolean".into()),
            },
            _ => return Err("Second argument expected to be a boolean".into()),
        };

        if restrain_empty && input.trim().is_empty() {
            return Err("Input cannot be empty".into());
        }
    }

    Ok(Expression::Literal {
        value: LiteralValue::Str(input.trim().to_owned()),
        raw: input,
    })
}
