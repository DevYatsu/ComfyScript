use crate::parser::ast::{Expression, ExpressionKind};
use nom::character::complete::char;
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::ParserExt;

use super::parse_basic_expression;

pub fn parse_opt_member_expr(
    initial_expr: Expression,
) -> impl Fn(&str) -> IResult<&str, Expression, ErrorTree<&str>> {
    move |i| {
        let (i, opt_dot) = char('.').opt().parse(i)?;

        if opt_dot.is_some() {
            let (i, final_expr) = parse_basic_expression.parse(i)?;

            return Ok((
                i,
                Expression::with_kind(ExpressionKind::MemberExpression {
                    indexed: Box::new(initial_expr.to_owned()),
                    property: Box::new(final_expr),
                    computed: false,
                }),
            ));
        }

        Ok((i, initial_expr.to_owned()))
    }
}
