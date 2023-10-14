// contains everything related to operations
// for instance assignement operators and binary operators
// as well as ways to build binary expressions

pub mod assignment;
pub mod binary;

use self::binary::BinaryOperator;
use crate::parser::ast::Expression;

pub fn build_binary_expression(
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
