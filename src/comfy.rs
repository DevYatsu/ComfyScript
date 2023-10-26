// standard library
mod collections;
mod env;
mod fs;
mod http;
mod io;
mod json;
mod math;
mod thread;
mod time;

use std::io as std_io;

use crate::parser::{
    ast::{literal_value::LiteralValue, Expression},
    expression::strings::StringFragment,
};

pub fn print(value: String) -> Expression {
    println!("{}", value);

    Expression::Literal {
        value: crate::parser::ast::literal_value::LiteralValue::Nil,
        raw: "nil".to_owned(),
    }
}

pub fn input(prompt: String) -> Expression {
    let mut input = String::new();

    print!("{}", prompt);

    if std_io::stdin().read_line(&mut input).is_ok() {
        let fragments = vec![StringFragment::Literal(input.clone())];
        Expression::OkExpression(Box::new(Expression::Literal {
            value: LiteralValue::Str(fragments),
            raw: input,
        }))
    } else {
        let fragments = vec![StringFragment::Literal("Failed to read line".to_owned())];
        Expression::ErrExpression(Box::new(Expression::Literal {
            value: LiteralValue::Str(fragments),
            raw: "Failed to read line".to_owned(),
        }))
    }
}
// or maybe the one below

// pub fn input(prompt: &str) -> Result<String, String> {
//     let mut input = String::new();

//     print!("{}", prompt);

//     if let Ok(_) = io::stdin().read_line(&mut input) {
//         Ok(input.trim().to_owned())
//     } else {
//         Err("Failed to read line".to_owned())
//     }
// }