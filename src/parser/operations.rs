pub mod assignment;
pub mod binary;

//todo! create parser for binary expressions
use nom::{
    branch::alt, character::complete::multispace0, combinator::opt, error::VerboseError, IResult,
};

use crate::parser::{ast::Expression, primitive_values::parse_primitive_value, Span};

use self::binary::{get_operator_precedence, parse_binary_operator};

use super::{composite_types::parse_composite_value, expression::parse_expression};

pub fn parse_binary_operation(input: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (input, mut expr) = alt((parse_composite_value, parse_primitive_value))(input)?;
    let (mut input, _) = multispace0(input)?;

    while let Ok((rest, mut operator)) = parse_binary_operator(input) {
        let precedence = get_operator_precedence(&operator);

        let (rest, _) = multispace0(rest)?;
        let (initial_rest, mut right) = alt((parse_composite_value, parse_primitive_value))(rest)?;
        let (rest, _) = multispace0(initial_rest)?;

        let (rest, mut other_op) = opt(parse_binary_operator)(rest)?;

        if other_op.is_none() {
            input = initial_rest;
            expr = Expression::BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };

            break;
        }

        while let Some(next_op) = other_op {
            let next_op_precedence = get_operator_precedence(&next_op);  

            if precedence >= next_op_precedence {
                let (rest, _) = multispace0(rest)?;
                let (rest, other) = parse_expression(rest)?;

                expr = Expression::BinaryExpression {
                    left: Box::new(Expression::BinaryExpression {
                        left: Box::new(expr),
                        operator,
                        right: Box::new(right),
                    }),
                    operator,
                    right: Box::new(other),
                };
                input = rest;
                break;
            } else {
                let (rest, _) = multispace0(rest)?;
                let (rest, next_right) = alt((parse_composite_value, parse_primitive_value))(rest)?;
                let (rest, _) = multispace0(rest)?;

                expr = Expression::BinaryExpression {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                };

                right = next_right;
                operator = next_op;

                let (new_i, other) = opt(parse_binary_operator)(rest)?;
                other_op = other;
                input = new_i;
            }
        }
    }

    Ok((input, expr))
}
