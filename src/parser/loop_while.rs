use super::{
    ast::ASTNode,
    errors::{expected_expression, expected_space},
    expression::parse_expression,
    parse_block,
};
use nom::{
    character::complete::{multispace0, multispace1},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_while_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, _) = tag("while")(input)?;
    let (input, _) = multispace1.context(expected_space()).parse(input)?;

    let (input, test) = parse_expression
        .context(expected_expression())
        .parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_block(input)?;

    let node = ASTNode::WhileStatement {
        test,
        body: Box::new(body),
    };

    Ok((input, node))
}
