// working with maths: cos, sin, tan, floor, round, ceil, PI constant etc

use lazy_static::lazy_static;

use crate::parser::ast::Expression;

lazy_static! {
    static ref PI: Expression = {
        Expression::Literal { value: crate::parser::ast::literal_value::LiteralValue::Number(std::f32::consts::PI), raw: std::f32::consts::PI.to_string() }
    };
}