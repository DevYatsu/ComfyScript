pub mod assignment;
pub mod binary;

//todo! create parser for binary expressions
use nom::{
    branch::alt, character::complete::multispace0, error::VerboseError, IResult,
};

use crate::parser::{ast::Expression, primitive_values::parse_primitive_value, Span};

use self::binary::{parse_binary_operator, BinaryOperator};

use super::composite_types::parse_composite_value;

pub fn parse_binary_operation(input: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let mut expr_vec = Vec::new();
    let mut operators_vec = Vec::new();

    let (input, expr) = alt((parse_composite_value, parse_primitive_value))(input)?;
    let (mut input, _) = multispace0(input)?;

    expr_vec.push(expr);

    loop {
        let (i, op) = parse_binary_operator(input)?;
        operators_vec.push(op);

        let (i, _) = multispace0(i)?;

        let (input_before_spaces_removed, expr) =
            alt((parse_composite_value, parse_primitive_value))(i)?;
        expr_vec.push(expr);

        let (i, _) = multispace0(input_before_spaces_removed)?;

        if let Err(_) = parse_binary_operator(i) {
            input = input_before_spaces_removed;
            break;
        }
        input = i;
    }

    let binary_expression = build_binary_expression(expr_vec, operators_vec);
    println!("{:?}", binary_expression);

    Ok((input, binary_expression))
}

fn build_binary_expression(
    expressions: Vec<Expression>,
    operators: Vec<BinaryOperator>,
) -> Expression {
    if expressions.len() == 1 {
        expressions.into_iter().next().unwrap()
    } else {
        // If there are multiple expressions, recursively build the binary expression
        let (first_op, remaining_ops) = operators.split_at(1);
        let (first_expr, remaining_exprs) = expressions.split_at(1);

        let binary_expr = Expression::BinaryExpression {
            left: Box::new(first_expr[0].clone()),
            operator: first_op[0],
            right: Box::new(build_binary_expression(
                remaining_exprs.to_vec(),
                remaining_ops.to_vec(),
            )),
        };

        binary_expr
    }
}