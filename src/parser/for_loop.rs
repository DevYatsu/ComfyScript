use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    error::VerboseError,
    IResult,
};

use crate::parser::utils::alpha_not_reserved;

use super::{ast::ASTNode, Span};

pub fn parse_for_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, identifier) = alphanumeric1(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("in")(input)?;
    let (input, _) = multispace0(input)?;

    let (input, indexed) = alpha_not_reserved(input)?; // todo!! call a fn to parse expression here
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("{")(input)?;
    let (input, _) = multispace0(input)?;

    // parse for content
    // todo!!
    unimplemented!();

    let (input, identifier) = tag("}")(input)?;
    Ok((input, ASTNode::ForStatement {}))
}
