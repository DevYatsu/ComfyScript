use crate::parser::ast::identifier::{parse_identifier, Identifier};
use crate::parser::ast::Expression;
use crate::parser::Span;
use nom::bytes::complete::tag;
use nom::error::VerboseError;
use nom::multi::separated_list1;
use nom::IResult;

pub fn parse_member_expr(i: Span) -> IResult<Span, Expression, VerboseError<Span>> {
    let (i, mut ids) = separated_list1(tag("."), parse_identifier)(i)?;
    // we are sure that ids length is >= 2 here

    let property = ids.pop().unwrap();

    if ids.len() == 0 {
        return Ok((i, Expression::IdentifierExpression(property)))
    }

    let object = if ids.len() == 1 {
        Box::new(Expression::IdentifierExpression(ids.remove(0)))
    } else {
        Box::new(build_member_expr(ids))
    };

    let expr = Expression::MemberExpression {
        object,
        property,
        computed: false,
    };
    Ok((i, expr))
}

fn build_member_expr(mut ids: Vec<Identifier>) -> Expression {
    if ids.len() == 1 {
        return Expression::IdentifierExpression(ids.remove(0));
    }

    let property = ids.pop().unwrap();

    let expr = Expression::MemberExpression {
        object: Box::new(build_member_expr(ids)),
        property,
        computed: false,
    };

    expr
}
