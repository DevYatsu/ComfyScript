// working with maths: cos, sin, tan, floor, round, ceil, PI constant etc

use lazy_static::lazy_static;
use rand::Rng;

use crate::parser::ast::literal_value::LiteralValue;
use crate::parser::ast::Expression;
use std::f32::consts;
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

pub fn cos(expr: f32) -> f32 {
    expr.cos()
}
pub fn sin(expr: f32) -> f32 {
    expr.sin()
}
pub fn tan(expr: f32) -> f32 {
    expr.tan()
}
pub fn acos(expr: f32) -> f32 {
    expr.acos()
}
pub fn asin(expr: f32) -> f32 {
    expr.asin()
}
pub fn atan(expr: f32) -> f32 {
    expr.atan()
}

pub fn ceil(expr: f32) -> f32 {
    expr.ceil()
}
pub fn floor(expr: f32) -> f32 {
    expr.floor()
}

pub fn log(expr: f32, base: f32) -> f32 {
    expr.log(base)
}
pub fn ln(expr: f32) -> f32 {
    expr.ln()
}

pub fn sqrt(expr: f32) -> f32 {
    expr.sqrt()
}
pub fn power(expr: f32, power: f32) -> f32 {
    expr.powf(power)
}

pub fn random() -> f32 {
    rand::thread_rng().gen()
}