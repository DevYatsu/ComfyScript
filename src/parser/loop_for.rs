use super::{
    assignment::initial::VariableKeyword,
    ast::{identifier::Identifier, ASTNode},
    expression::parse_expression,
    parse_block,
};
use crate::parser::ast::identifier::parse_identifier;
use nom::{
    branch::alt,
    character::complete::{multispace0, multispace1},
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_for_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, _) = tag("for").complete().parse(input)?;
    let (input, _) = multispace1.parse(input)?;

    let (input, kind) = parse_for_var_keyword(input)?;

    let (input, identifiers) = separated_list1(tag(","), parse_for_identifier)
        .cut()
        .parse(input)?;

    let (input, _) = tag(",").preceded_by(multispace0).opt().parse(input)?;

    let (input, _) = multispace1.cut().parse(input)?;

    let (input, _) = tag("in").complete().cut().parse(input)?;

    let (input, _) = multispace1.cut().parse(input)?;

    let (input, indexed) = parse_expression(input)?;

    let (input, _) = multispace0(input)?;

    let (input, body) = parse_block.map(|b| Box::new(b)).parse(input)?;

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

    let (input, id) = parse_identifier(input)?;

    Ok((input, id))
}

fn parse_for_var_keyword(input: &str) -> IResult<&str, VariableKeyword, ErrorTree<&str>> {
    let (input, var_keyword) = alt((
        tag("let").complete().map(|_| VariableKeyword::Let),
        tag("var").complete().map(|_| VariableKeyword::Var),
    ))
    .opt()
    .parse(input)?;

    let input = if var_keyword.is_some() {
        multispace1(input)?.0
    } else {
        input
    };

    Ok((input, var_keyword.unwrap_or(VariableKeyword::Let)))
}
