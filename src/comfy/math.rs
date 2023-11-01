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
        "cos" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(cos(value)),
        },
        "sin" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(sin(value)),
        },
        "tan" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(tan(value)),
        },
        "acos" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(acos(value)),
        },
        "asin" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(asin(value)),
        },
        "atan" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(atan(value)),
        },
        "ceil" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(ceil(value)),
        },
        "floor" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(floor(value)),
        },
        "log" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(log(value)),
        },
        "ln" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(ln(value)),
        },
        "sqrt" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(sqrt(value)),
        },
        "power" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(power(value)),
        },
        "random" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(random(value)),
        },
        "abs" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(abs(value)),
        },
        "exp" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(exp(value)),
        },
        "max" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(max(value)),
        },
        "min" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(min(value)),
        },
        "round" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(round(value)),
        },
        "trunc" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(trunc(value)),
        },
        "clamp" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(clamp(value)),
        },
        "signum" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(signum(value)),
        },
        "to_radians" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(to_radians(value)),
        },
        "to_degrees" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(to_degrees(value)),
        },
        _ => {
            return Err(format!(
                "'math' package does not export a `{}` member",
                value
            ))
        }
    };

    Ok(result)
}

fn cos(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.cos();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn sin(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.sin();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn tan(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.tan();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn acos(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.acos();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn asin(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.asin();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn atan(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.atan();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn ceil(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.ceil();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn floor(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.floor();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn log(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 2, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let base: f32 = args[1].to_owned().into();

        let result = num.log(base);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn ln(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.ln();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}

fn sqrt(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.sqrt();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn power(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 2, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let power: f32 = args[1].to_owned().into();

        let result = num.powf(power);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn random(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |s: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        sanitize_math_args(s, &value, 0, args)?;

        let result = rand::thread_rng().gen();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn abs(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.abs();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn exp(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.exp();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn max(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 2, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let max: f32 = args[1].to_owned().into();

        let result = num.max(max);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn min(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 2, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let min: f32 = args[1].to_owned().into();

        let result = num.min(min);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn round(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.round();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn trunc(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.trunc();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn clamp(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 3, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
        let min: f32 = args[1].to_owned().into();
        let max: f32 = args[2].to_owned().into();

        let result = num.clamp(min, max);

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn signum(value: String) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.signum();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn to_radians(
    value: String,
) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

        let result = num.to_radians();

        Ok((LiteralValue::Number(result), result.to_string()).into())
    }
}
fn to_degrees(
    value: String,
) -> impl Fn(&SymbolTable, Vec<Expression>) -> Result<Expression, String> {
    move |symbol_table: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

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
