use crate::parser::ast::range::RangeType;
use crate::parser::ast::{Expression, ExpressionKind};
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
            value(RangeType::Dot, tag("..").complete()),
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
                (
                    Some(Box::new(initial_expr.to_owned())),
                    range_type,
                    final_expr,
                )
                    .into(),
            ));
        }

        Ok((i, initial_expr.to_owned()))
    }
}

pub fn parse_range(i: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (i, range_type) = alt((
        value(RangeType::DotEqual, tag("..=").complete()),
        value(RangeType::Dot, tag("..").complete()),
    ))
    .preceded_by(multispace0)
    .parse(i)?;

    let (i, final_expr) = parse_basic_expression
        .preceded_by(multispace0)
        .opt()
        .parse(i)?;

    if let Some(final_expr) = final_expr {
        return Ok((i, (None, range_type, Some(Box::new(final_expr))).into()));
    }

    Ok((i, (None, range_type, None).into()))
}

impl Into<Expression> for (Option<Box<Expression>>, RangeType, Option<Box<Expression>>) {
    fn into(self) -> Expression {
        Expression::with_kind(ExpressionKind::Range(self.0, self.1, self.2))
    }
}
