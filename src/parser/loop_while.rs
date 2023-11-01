use super::{
    ast::{Statement, StatementKind},
    expression::parse_expression,
    parse_block,
};
use nom::{
    character::complete::{multispace0, multispace1},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, ParserExt};

pub fn parse_while_statement(input: &str) -> IResult<&str, Statement, ErrorTree<&str>> {
    let (input, _) = multispace1.cut().parse(input)?;

    let (input, test) = parse_expression.parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_block.cut().parse(input)?;

    let node = StatementKind::WhileStatement(test, body);

    Ok((input, node.into()))
}
