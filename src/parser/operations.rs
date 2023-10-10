pub mod assignment;
pub mod binary;

use nom::{branch::alt, character::complete::multispace0, error::VerboseError, IResult};

use crate::parser::{
    parenthesized::parse_parenthesized, primitive_values::parse_primitive_value, Span,
};

use self::binary::{parse_binary_operator, BinaryOperator};

use super::{ast::Expression, composite_types::parse_composite_value};

pub fn parse_binary_operation(input: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let mut expr_vec = Vec::with_capacity(4); // Assuming a small initial capacity
    let mut operators_vec = Vec::with_capacity(3); // Assuming a small initial capacity

    let (input, expr) = alt((
        parse_composite_value,
        parse_primitive_value,
        parse_parenthesized,
    ))(input)?;

    expr_vec.push(expr);

    let (mut input, _) = multispace0(input)?;

    while let Ok((i, op)) = parse_binary_operator(input) {
        operators_vec.push(op);

        let (i, _) = multispace0(i)?;

        let (input_before_spaces_removed, expr) = alt((
            parse_composite_value,
            parse_primitive_value,
            parse_parenthesized,
        ))(i)?;

        expr_vec.push(expr);

        let (i, _) = multispace0(input_before_spaces_removed)?;

        input = if let Ok(_) = parse_binary_operator(i) {
            i
        } else {
            input_before_spaces_removed
        };
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
            .rev()
            .max_by_key(|(_, op)| op.get_precedence())
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

    expressions.pop().unwrap()
}
