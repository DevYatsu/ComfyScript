// working with maths: cos, sin, tan, floor, round, ceil, PI constant etc

use lazy_static::lazy_static;
use rand::Rng;

use crate::interpreter::{InterpretedFn, SymbolTable};
use crate::parser::ast::literal_value::LiteralValue;
use crate::parser::ast::Expression;
use std::f32::consts;
use std::rc::Rc;

use super::{expected_number_arg, expected_x_args};

lazy_static! {
    static ref PI: Expression = (LiteralValue::Number(consts::PI), consts::PI.to_string(),).into();
    static ref FRAC_1_PI: Expression = (
        LiteralValue::Number(consts::FRAC_1_PI),
        consts::FRAC_1_PI.to_string(),
    )
        .into();
    static ref E: Expression = (LiteralValue::Number(consts::E), consts::E.to_string(),).into();
    static ref LN_10: Expression = (
        LiteralValue::Number(consts::LN_10),
        consts::LN_10.to_string(),
    )
        .into();
    static ref LN_2: Expression =
        (LiteralValue::Number(consts::LN_2), consts::LN_2.to_string(),).into();
    static ref LOG10_2: Expression = (
        LiteralValue::Number(consts::LOG10_2),
        consts::LOG10_2.to_string(),
    )
        .into();
    static ref SQRT_2: Expression = (
        LiteralValue::Number(consts::SQRT_2),
        consts::SQRT_2.to_string(),
    )
        .into();
}

pub fn import_math_fn(value: String) -> Result<InterpretedFn, String> {
    let result = match value.as_str() {
        "cos" => InterpretedFn(Rc::new(cos())),
        "sin" => InterpretedFn(Rc::new(sin())),
        "tan" => InterpretedFn(Rc::new(tan())),
        "acos" => InterpretedFn(Rc::new(acos())),
        "asin" => InterpretedFn(Rc::new(asin())),
        "atan" => InterpretedFn(Rc::new(atan())),
        "ceil" => InterpretedFn(Rc::new(ceil())),
        "floor" => InterpretedFn(Rc::new(floor())),
        "log" => InterpretedFn(Rc::new(log())),
        "ln" => InterpretedFn(Rc::new(ln())),
        "sqrt" => InterpretedFn(Rc::new(sqrt())),
        "power" => InterpretedFn(Rc::new(power())),
        "random" => InterpretedFn(Rc::new(random())),
        "abs" => InterpretedFn(Rc::new(abs())),
        "exp" => InterpretedFn(Rc::new(exp())),
        "max" => InterpretedFn(Rc::new(max())),
        "min" => InterpretedFn(Rc::new(min())),
        "round" => InterpretedFn(Rc::new(round())),
        "trunc" => InterpretedFn(Rc::new(trunc())),
        "clamp" => InterpretedFn(Rc::new(clamp())),
        "signum" => InterpretedFn(Rc::new(signum())),
        "to_radians" => InterpretedFn(Rc::new(to_radians())),
        "to_degrees" => InterpretedFn(Rc::new(to_degrees())),
        _ => {
            return Err(format!(
                "'math' package does not export a `{}` member",
                value
            ))
        }
    };

    Ok(result)
}

fn cos() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "cos", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.cos();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn sin() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "sin", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.sin();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn tan() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "tan", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.tan();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn acos() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "acos", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.acos();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn asin() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "asin", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.asin();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn atan() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "atan", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.atan();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn ceil() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "ceil", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.ceil();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn floor() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "floor", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.floor();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn log() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "log", 2, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let base: f32 = args[1].to_owned().into();

        let result = num.log(base);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn ln() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "ln", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.ln();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn sqrt() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "sqrt", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.sqrt();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn power() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "power", 2, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let power: f32 = args[1].to_owned().into();

        let result = num.powf(power);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn random() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |s: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        sanitize_math_args(s, "random", 0, args)?;

        let result = rand::thread_rng().gen();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn abs() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "abs", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.abs();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn exp() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "exp", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.exp();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn max() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "max", 2, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let max: f32 = args[1].to_owned().into();

        let result = num.max(max);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn min() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "min", 2, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let min: f32 = args[1].to_owned().into();

        let result = num.min(min);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn round() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "round", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.round();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn trunc() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "trunc", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.trunc();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn clamp() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "clamp", 3, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let min: f32 = args[1].to_owned().into();
        let max: f32 = args[2].to_owned().into();

        let result = num.clamp(min, max);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn signum() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "signum", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.signum();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn to_radians() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "to_radians", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.to_radians();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn to_degrees() -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, "to_degrees", 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.to_degrees();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn sanitize_math_args(
    symbol_table: &SymbolTable,
    fn_name: &str,
    expected_length: usize,
    args: Vec<Expression>,
) -> Result<Vec<Expression>, String> {
    expected_x_args(fn_name, expected_length, &args)?;

    let args = args
        .into_iter()
        .map(|arg| {
            let arg = expected_number_arg(symbol_table, fn_name, arg)?;

            Ok(arg)
        })
        .collect::<Result<Vec<Expression>, String>>()?;

    Ok(args)
}
