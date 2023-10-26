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

use crate::parser::{ast::Expression, expression::strings::StringFragment};

pub fn print(value: String) -> Expression {
    println!("{}", value);

    Expression::Literal {
        value: crate::parser::ast::literal_value::LiteralValue::Nil,
        raw: "nil".to_owned(),
    }
}

pub fn input(value: String) -> Expression {
    let mut input = String::new();

    std_io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let fragments = vec![StringFragment::Literal(input.to_owned())];
    Expression::Literal {
        value: crate::parser::ast::literal_value::LiteralValue::Str(fragments),
        raw: input,
    }
}
