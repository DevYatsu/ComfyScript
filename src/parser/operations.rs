pub mod assignment;
pub mod binary;

//todo! create parser for binary expressions
use nom::{branch::alt, character::complete::multispace0, error::VerboseError, IResult};

use crate::parser::{
    ast::Expression, operations::binary::get_operator_precedence,
    primitive_values::parse_primitive_value, Span,
};

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
    mut expressions: Vec<Expression>,
    mut operators: Vec<BinaryOperator>,
) -> Expression {
    while !operators.is_empty() {
        let max_precedence_index = operators
            .iter()
            .enumerate()
            .max_by_key(|(_, op)| get_operator_precedence(op))
            .map(|(index, _)| index);

        if let Some(index) = max_precedence_index {
            let operator = operators.remove(index);
            let right = expressions.remove(index + 1);
            let left = expressions.remove(index);

            let binary_op = Expression::BinaryExpression {
                left: Box::new(left),
                operator,
                right: Box::new(right),
            };

            expressions.insert(index, binary_op);
        }
    }

    expressions.remove(0)
}
