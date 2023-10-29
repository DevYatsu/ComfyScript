// working with maths: cos, sin, tan, floor, round, ceil, PI constant etc

use lazy_static::lazy_static;
use rand::Rng;

use crate::interpreter::{InterpretedFn, SymbolTable};
use crate::parser::ast::literal_value::LiteralValue;
use crate::parser::ast::Expression;
use std::f32::consts;
use std::rc::Rc;
lazy_static! {
    static ref PI: Expression = {
        Expression::Literal {
            value: LiteralValue::Number(consts::PI),
            raw: consts::PI.to_string(),
        }
    };
    static ref FRAC_1_PI: Expression = {
        Expression::Literal {
            value: LiteralValue::Number(consts::FRAC_1_PI),
            raw: consts::FRAC_1_PI.to_string(),
        }
    };
    static ref E: Expression = {
        Expression::Literal {
            value: LiteralValue::Number(consts::E),
            raw: consts::E.to_string(),
        }
    };
    static ref LN_10: Expression = {
        Expression::Literal {
            value: LiteralValue::Number(consts::LN_10),
            raw: consts::LN_10.to_string(),
        }
    };
    static ref LN_2: Expression = {
        Expression::Literal {
            value: LiteralValue::Number(consts::LN_2),
            raw: consts::LN_2.to_string(),
        }
    };
    static ref LOG10_2: Expression = {
        Expression::Literal {
            value: LiteralValue::Number(consts::LOG10_2),
            raw: consts::LOG10_2.to_string(),
        }
    };
    static ref SQRT_2: Expression = {
        Expression::Literal {
            value: LiteralValue::Number(consts::SQRT_2),
            raw: consts::SQRT_2.to_string(),
        }
    };
}

pub fn import_math_fn(value: String) -> Result<InterpretedFn, String> {
    let result = match value.as_str() {
        "cos" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.cos();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "sin" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.sin();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "tan" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.tan();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "acos" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.acos();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "asin" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.asin();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "atan" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.atan();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "ceil" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.ceil();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "floor" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.floor();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "log" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 2, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
                        let base: f32 = args[1].to_owned().into();

                        let result = num.log(base);

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "ln" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.ln();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "sqrt" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.sqrt();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "power" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 2, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
                        let power: f32 = args[1].to_owned().into();

                        let result = num.powf(power);

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "random" => InterpretedFn {
            name: value.to_owned(),
            executable: Rc::new(
                move |s: &SymbolTable, args: Vec<Expression>| -> Result<Expression, String> {
                    sanitize_math_args(s, &value, 0, args)?;

                    let result = rand::thread_rng().gen();

                    Ok(Expression::Literal {
                        value: LiteralValue::Number(result),
                        raw: result.to_string(),
                    })
                },
            ),
        },
        "abs" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.abs();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "exp" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.exp();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "max" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 2, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
                        let max: f32 = args[1].to_owned().into();

                        let result = num.max(max);

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "min" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 2, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
                        let min: f32 = args[1].to_owned().into();

                        let result = num.min(min);

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "round" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.round();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "trunc" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.trunc();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "clamp" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 3, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();
                        let min: f32 = args[1].to_owned().into();
                        let max: f32 = args[2].to_owned().into();

                        let result = num.clamp(min, max);

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "signum" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.signum();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "to_radians" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.to_radians();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        "to_degrees" => {
            InterpretedFn {
                name: value.to_owned(),
                executable: Rc::new(
                    move |symbol_table: &SymbolTable,
                          args: Vec<Expression>|
                          -> Result<Expression, String> {
                        let args = sanitize_math_args(symbol_table, &value, 1, args)?;

                        // here we can do that only because we check up here
                        // that args are all Number Expressions
                        let num: f32 = symbol_table.evaluate_expr(args[0].to_owned())?.into();

                        let result = num.to_degrees();

                        Ok(Expression::Literal {
                            value: LiteralValue::Number(result),
                            raw: result.to_string(),
                        })
                    },
                ),
            }
        }
        _ => return Err(format!("Math function '{}' not found", value)),
    };

    Ok(result)
}

fn sanitize_math_args(
    symbol_table: &SymbolTable,
    fn_name: &str,
    expected_length: usize,
    args: Vec<Expression>,
) -> Result<Vec<Expression>, String> {
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

    let args = args
        .into_iter()
        .map(|arg| {
            let arg = symbol_table.evaluate_expr(arg)?;
            match &arg {
                Expression::Literal { value, .. } => match value {
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
        })
        .collect::<Result<Vec<Expression>, String>>()?;

    Ok(args)
}
