use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char as parse_char, multispace0, multispace1},
    combinator::opt,
    error::VerboseError,
    multi::separated_list1,
    sequence::delimited,
    IResult,
};

use crate::parser::ast::identifier::parse_identifier;

use super::{
    ast::{identifier::Identifier, ASTNode},
    expression::parse_expression,
    parse_block, Span,
};

pub fn parse_function(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, is_anonymous) = opt(tag("anon"))(input)?;

    let (input, is_anonymous) = match is_anonymous {
        Some(_) => {
            let (input, _) = multispace1(input)?;

            (input, true)
        }
        None => (input, false),
    };

    let (input, _) = tag("fn")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, id) = parse_identifier(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = parse_char('(')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, params) = separated_list1(tag(","), parse_fn_identifier)(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = parse_char(')')(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("{")(input)?;

    let (input, body) = parse_block(input, "}")?;

    let node = ASTNode::FunctionDeclaration {
        id,
        params,
        body,
        is_anonymous,
    };

    Ok((input, node))
}

fn parse_fn_identifier(input: Span) -> IResult<Span, Identifier, VerboseError<Span>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier(input)?;

    Ok((input, id))
}
