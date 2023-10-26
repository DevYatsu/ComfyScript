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
            value(RangeType::DotEqual, tag("..=").complete()),
            value(RangeType::Dot, tag("..")).complete(),
        ))
        .preceded_by(multispace0)
        .opt()
        .parse(i)?;

        if let Some(range_type) = opt_range_type {
            let (i, final_expr) = parse_basic_expression
                .preceded_by(multispace0)
                .map(Box::new)
                .opt()
                .parse(i)?;

            return Ok((
                i,
                Expression::Range {
                    from: Some(Box::new(initial_expr.to_owned())),
                    limits: range_type,
                    to: final_expr,
                },
            ));
        }

        Ok((i, initial_expr.to_owned()))
    }
}

pub fn parse_range(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, range_type) = alt((
        value(RangeType::DotEqual, tag("..=").complete()),
        value(RangeType::Dot, tag("..")).complete(),
    ))
    .preceded_by(multispace0)
    .parse(i)?;

    let (i, final_expr) = parse_basic_expression.preceded_by(multispace0).parse(i)?;

    Ok((
        i,
        Expression::Range {
            from: None,
            limits: range_type,
            to: Some(Box::new(final_expr)),
        },
    ))
}
