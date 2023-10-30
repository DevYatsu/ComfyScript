// working with time

use std::{rc::Rc, time::Duration};

use crate::{
    interpreter::{InterpretedFn, SymbolTable},
    parser::ast::{literal_value::LiteralValue, Expression},
};

use super::{expected_number_arg, expected_x_args};

pub fn import_time_fn(name: String) -> Result<InterpretedFn, String> {
    let result = match name.as_str() {
        "sleep" => InterpretedFn {
            name: name.to_owned(),
            executable: Rc::new(sleep()),
        },

        _ => {
            return Err(format!(
                "'time' package does not export a `{}` member",
                name
            ))
        }
    };

    Ok(result)
}

fn sleep() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        expected_x_args("sleep", 1, &args)?;

        let num: f32 = expected_number_arg(symbol_table, "sleep", args[0].to_owned())?.into();

        let dur = Duration::from_millis(num as u64);

        std::thread::sleep(dur);
        Ok(Expression::Literal {
            value: LiteralValue::Nil,
            raw: "nil".to_owned(),
        })
    }
}
