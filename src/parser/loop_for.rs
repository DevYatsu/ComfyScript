use super::{
    assignment::initial::VariableKeyword,
    ast::{identifier::Identifier, ASTNode},
    expression::parse_expression,
    parse_block, Span,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    error::VerboseError,
    multi::separated_list1,
    IResult,
};

pub fn parse_for_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, opt_keyword) = opt(alt((tag("let"), tag("var"))))(input)?;

    let (input, keyword) = if let Some(k) = opt_keyword {
        let (input, _) = multispace1(input)?;

        (
            input,
            match k.fragment() {
                &"var" => VariableKeyword::Var,
                _ => VariableKeyword::Let,
            },
        )
    } else {
        (input, VariableKeyword::Let)
    };

    let (input, identifiers) = separated_list1(tag(","), parse_for_identifier)(input)?;
    let (input, _) = multispace1(input)?;

    let (input, _) = tag("in")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, indexed) = parse_expression(input)?;

    let (input, _) = multispace0(input)?;

    let (input, body) = map(parse_block, |b| Box::new(b))(input)?;

    let node = ASTNode::ForStatement {
        kind: keyword,
        declarations: identifiers,
        source: indexed,
        body,
    };

    Ok((input, node))
}

fn parse_for_identifier(input: Span) -> IResult<Span, Identifier, VerboseError<Span>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier(input)?;

    Ok((input, id))
}
