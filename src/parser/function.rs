pub mod function_call;
pub mod return_expression;

use self::return_expression::parse_return_statement;

use super::{
    ast::{identifier::Identifier, ASTNode},
    parse_block, Span,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    bytes::complete::tag,
    character::complete::{char as parse_char, multispace0, multispace1},
    combinator::opt,
    error::VerboseError,
    multi::separated_list1,
    IResult,
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

    let (input, params) = opt(separated_list1(tag(","), parse_fn_identifier))(input)?;
    let params = params.unwrap_or_else(|| vec![]);
    let (input, _) = multispace0(input)?;

    let (input, _) = parse_char(')')(input)?;
    let (input, _) = multispace0(input)?;

    let (_, instant_return) = opt(tag(">>"))(input)?;

    if instant_return.is_some() {
        let (input, return_statement) = parse_return_statement(input)?;

        let node = ASTNode::FunctionDeclaration {
            id,
            params,
            body: Box::new(return_statement),
            is_anonymous,
            is_shortcut: false,
        };

        return Ok((input, node));
    }

    let (input, _) = tag("{")(input)?;

    let (input, body) = parse_block(input, "}")?;

    let node = ASTNode::FunctionDeclaration {
        id,
        params,
        body: Box::new(body),
        is_anonymous,
        is_shortcut: false,
    };

    Ok((input, node))
}

fn parse_fn_identifier(input: Span) -> IResult<Span, Identifier, VerboseError<Span>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier(input)?;

    Ok((input, id))
}
