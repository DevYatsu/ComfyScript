use super::{
    ast::{ASTNode, Expression},
    errors::{expected_expression, expected_space},
    expression::parse_expression,
    parse_block,
};
use nom::{
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_if_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, (test, body)) = parse_if_block(input)?;

    let (else_input, _) = multispace0(input)?;
    let (else_input, else_word) = opt(tag("else"))(else_input)?;

    if else_word.is_none() {
        let node = ASTNode::IfStatement {
            test,
            body,
            alternate: None,
        };

        return Ok((input, node));
    }

    let (else_input, _) = multispace0(else_input)?;
    let (_, other_if) = opt(tag("if"))(else_input)?;

    if other_if.is_none() {
        let (else_input, alternate) = map(parse_block, |s| Some(Box::new(s)))(else_input)?;

        let node = ASTNode::IfStatement {
            test,
            body,
            alternate,
        };

        return Ok((else_input, node));
    }

    let (else_input, _) = multispace0(else_input)?;

    let (input, alternate) = map(parse_if_statement, |s| Some(Box::new(s)))(else_input)?;

    let node = ASTNode::IfStatement {
        test,
        body,
        alternate,
    };

    Ok((input, node))
}

fn parse_if_block(input: &str) -> IResult<&str, (Expression, Box<ASTNode>), ErrorTree<&str>> {
    let (input, _) = tag("if")(input)?;
    let (input, _) = multispace1.context(expected_space()).parse(input)?;

    let (input, test) = parse_expression
        .context(expected_expression())
        .parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = map(parse_block, |b| Box::new(b))(input)?;

    Ok((input, (test, body)))
}
