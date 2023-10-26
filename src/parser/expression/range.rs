use crate::parser::ast::range::RangeType;
use crate::parser::ast::Expression;
use nom::branch::alt;
use nom::character::complete::multispace0;
use nom::combinator::value;
use nom::{IResult, Parser};
use nom_supreme::ParserExt;
use nom_supreme::{error::ErrorTree, tag::complete::tag};

use super::parse_basic_expression;

pub fn parse_opt_range(
    initial_expr: Expression,
) -> impl Fn(&str) -> IResult<&str, Expression, ErrorTree<&str>> {
    move |i| {
        let (i, opt_range_type) = alt((
            value(RangeType::Dot, tag("..")),
            value(RangeType::DotEqual, tag("..=")),
        ))
        .preceded_by(multispace0)
        .opt()
        .parse(i)?;

        if let Some(range_type) = opt_range_type {
            let (i, final_expr) = parse_basic_expression.preceded_by(multispace0).parse(i)?;

            return Ok((
                i,
                Expression::Range {
                    from: Box::new(initial_expr.to_owned()),
                    limits: range_type,
                    to: Box::new(final_expr),
                },
            ));
        }

        Ok((i, initial_expr.to_owned()))
    }
}
