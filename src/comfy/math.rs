// working with maths: cos, sin, tan, floor, round, ceil, PI constant etc

use lazy_static::lazy_static;

use crate::parser::ast::Expression;
use std::f32::consts;
use crate::parser::ast::literal_value::LiteralValue;
lazy_static! {
    static ref PI: Expression = {
        Expression::Literal { value: LiteralValue::Number(consts::PI), raw: consts::PI.to_string() }
    };
    static ref FRAC_1_PI: Expression = {
        Expression::Literal { value: LiteralValue::Number(consts::FRAC_1_PI), raw: consts::FRAC_1_PI.to_string() }
    };
    static ref E: Expression = {
        Expression::Literal { value: LiteralValue::Number(consts::E), raw: consts::E.to_string() }
    };
    static ref LN_10: Expression = {
        Expression::Literal { value: LiteralValue::Number(consts::LN_10), raw: consts::LN_10.to_string() }
    };
    static ref LN_2: Expression = {
        Expression::Literal { value: LiteralValue::Number(consts::LN_2), raw: consts::LN_2.to_string() }
    };
    static ref LOG10_2: Expression = {
        Expression::Literal { value: LiteralValue::Number(consts::LOG10_2), raw: consts::LOG10_2.to_string() }
    };
    static ref SQRT_2: Expression = {
        Expression::Literal { value: LiteralValue::Number(consts::SQRT_2), raw: consts::SQRT_2.to_string() }
    };
}