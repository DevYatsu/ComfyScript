use super::{
    assignment::initial::VariableKeyword,
    ast::{identifier::Identifier, ASTNode},
    expression::parse_expression,
    parse_block, errors::{expected_space, expected_expression},
};
use crate::{parser::ast::identifier::parse_identifier, expected_keyword, expected_valid};
use nom::{
    branch::alt,
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_for_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, _) = tag("for")(input)?;
    let (input, _) = multispace1.context(expected_space()).parse(input)?;

    let (input, kind) = parse_for_var_keyword(input)?;

    let (input, identifiers) = separated_list1(tag(","), parse_for_identifier)(input)?;
    let (input, _) = opt(tag(","))(input)?;
    let (input, _) = multispace1.context(expected_space()).parse(input)?;

    let (input, _) = tag("in").context(expected_keyword!("in")).parse(input)?;
    let (input, _) = multispace1.context(expected_space()).parse(input)?;

    let (input, indexed) = parse_expression.context(expected_expression()).parse(input)?;

    let (input, _) = multispace0(input)?;

    let (input, body) = map(parse_block, |b| Box::new(b))(input)?;

    let node = ASTNode::ForStatement {
        kind,
        declarations: identifiers,
        source: indexed,
        body,
    };

    Ok((input, node))
}

fn parse_for_identifier(input: &str) -> IResult<&str, Identifier, ErrorTree<&str>> {
    let (input, _) = multispace0(input)?;

    let (input, id) = parse_identifier.context(expected_valid!("identifier")).parse(input)?;

    Ok((input, id))
}

fn parse_for_var_keyword(input: &str) -> IResult<&str, VariableKeyword, ErrorTree<&str>> {
    let (input, var_keyword) = opt(alt((
        map(tag("let"), |_| VariableKeyword::Let),
        map(tag("var"), |_| VariableKeyword::Var),
    )))(input)?;

    let input = if var_keyword.is_some() {
        multispace1(input)?.0
    } else {
        input
    };

    Ok((input, var_keyword.unwrap_or(VariableKeyword::Let)))
}
