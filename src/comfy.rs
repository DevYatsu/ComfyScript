// standard library
pub mod collections;
pub mod env;
pub mod fs;
pub mod http;
pub mod input_output;
pub mod json;
pub mod math;
pub mod thread;
pub mod time;

use std::{
    io::{self, Write},
    rc::Rc,
};

use hashbrown::HashMap;

use crate::{
    interpreter::{InterpretedFn, SymbolTable},
    parser::ast::{literal_value::LiteralValue, Expression, ExpressionKind},
};

pub fn init_std_functions(functions_hash: &mut HashMap<String, Rc<InterpretedFn>>) {
    functions_hash.insert("print".to_owned(), Rc::new(InterpretedFn(Rc::new(print()))));
    functions_hash.insert("input".to_owned(), Rc::new(InterpretedFn(Rc::new(input()))));
}

pub fn print() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        if args.len() != 1 {
            return Err("Expected 1 argument for function `print`".into());
        }

        let arg = args[0].to_owned();

        let print = match arg.kind {
            ExpressionKind::Err(_) => arg.kind.console_print(),
            ExpressionKind::Ok(_) => arg.kind.console_print(),
            _ => symbol_table.evaluate_expr(arg)?.console_print(),
        };

        println!("{}", print);

        Ok((LiteralValue::Nil, "nil".to_owned()).into())
    }
}

pub fn input() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
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
            let restrain_empty = match symbol_table.evaluate_expr(args[1].to_owned())?.kind {
                ExpressionKind::Literal(value, ..) => match value {
                    LiteralValue::Boolean(b) => b,
                    _ => return Err("Second argument expected to be a boolean".into()),
                },
                _ => return Err("Second argument expected to be a boolean".into()),
            };

            if restrain_empty && input.trim().is_empty() {
                return Err("Input cannot be empty".into());
            }
        }

        Ok((LiteralValue::Str(input.trim().to_owned()), input).into())
    }
}

fn expected_x_args(
    fn_name: &str,
    expected_length: usize,
    args: &Vec<Expression>,
) -> Result<(), String> {
    if args.len() != expected_length {
        if expected_length < 2 {
            return Err(format!(
                "Expected {} argument for function `{}`",
                expected_length, fn_name
            ));
        } else {
            return Err(format!(
                "Expected {} arguments for function `{}`",
                expected_length, fn_name
            ));
        }
    }

    Ok(())
}

fn expected_number_arg(
    symbol_table: &SymbolTable,
    fn_name: &str,
    arg: Expression,
) -> Result<Expression, String> {
    let arg = symbol_table.evaluate_expr(arg)?;

    match &arg.kind {
        ExpressionKind::Literal(value, ..) => match value {
            LiteralValue::Number(_) => (),
            _ => {
                return Err(format!(
                    "Expected arguments of type 'Number' for function `{}`",
                    fn_name
                ))
            }
        },
        _ => {
            return Err(format!(
                "Expected arguments of type 'Number' for function `{}`",
                fn_name
            ))
        }
    }

    Ok(arg)
}

fn expected_string_arg(
    symbol_table: &SymbolTable,
    fn_name: &str,
    arg: Expression,
) -> Result<Expression, String> {
    let arg = symbol_table.evaluate_expr(arg)?;

    match &arg.kind {
        ExpressionKind::Literal(value, ..) => match value {
            LiteralValue::Str(_) => (),
            value => {
                return Err(format!(
                    "Expected arguments of type 'String' for function `{}`, found type `{}`",
                    fn_name,
                    value.get_type()
                ))
            }
        },
        value => {
            return Err(format!(
                "Expected arguments of type 'String' for function `{}`, found type `{}`",
                fn_name,
                value.clone().get_type()
            ))
        }
    }

    Ok(arg)
}
