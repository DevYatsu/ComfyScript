use crate::parser::ast::identifier::parse_identifier_expression;
use crate::parser::ast::Expression;
use nom::branch::alt;
use nom::character::complete::{alphanumeric0, char, multispace0};
use nom::multi::separated_list1;
use nom::{IResult, Parser};
use nom_supreme::error::ErrorTree;
use nom_supreme::tag::complete::tag;
use nom_supreme::ParserExt;

use super::array::parse_array;
use super::function_call::parse_fn_call;
use super::indexing::parse_indexing;
use super::numbers::parse_number;
use super::object::parse_object;
use super::parenthesized::parse_parenthesized;
use super::strings::parse_string;
use super::template_literal::parse_template_literal;
use super::{parse_basic_expression, parse_expression_with0};

pub fn parse_opt_member_expr(
    initial_expr: Expression,
) -> impl Fn(&str) -> IResult<&str, Expression, ErrorTree<&str>> {
    move |i| {
        let (i, opt_dot) = char('.').opt().parse(i)?;

        if opt_dot.is_some() {
            let (i, final_expr) = parse_basic_expression.parse(i)?;

            return Ok((
                i,
                Expression::MemberExpression {
                    indexed: Box::new(initial_expr.to_owned()),
                    property: Box::new(final_expr),
                    computed: false,
                },
            ));
        }

        Ok((i, initial_expr.to_owned()))
    }
}
