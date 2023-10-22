use super::{
    ast::{ASTNode, Expression},
    expression::parse_expression,
    parse_block,
};
use nom::{
    character::complete::{multispace0, multispace1},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_if_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, (test, body)) = parse_if_block(input)?;

    let (else_input, _) = multispace0(input)?;
    let (else_input, else_word) = tag("else").opt().parse(else_input)?;

    if else_word.is_none() {
        let node = ASTNode::IfStatement {
            test,
            body,
            alternate: None,
        };

        return Ok((input, node));
    }

    let (else_input, _) = multispace0(else_input)?;
    let (_, other_if) = tag("if").opt().parse(else_input)?;

    if other_if.is_none() {
        let (else_input, alternate) = parse_block.map(|s| Some(Box::new(s))).parse(else_input)?;

        let node = ASTNode::IfStatement {
            test,
            body,
            alternate,
        };

        return Ok((else_input, node));
    }

    let (else_input, _) = multispace0(else_input)?;

    let (input, alternate) = parse_if_statement
        .map(|s| Some(Box::new(s)))
        .parse(else_input)?;

    let node = ASTNode::IfStatement {
        test,
        body,
        alternate,
    };

    Ok((input, node))
}

fn parse_if_block(input: &str) -> IResult<&str, (Expression, Box<ASTNode>), ErrorTree<&str>> {
    let (input, _) = tag("if").complete().parse(input)?;
    let (input, _) = multispace1.cut().parse(input)?;

    let (input, test) = parse_expression.cut().parse(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_block.cut().map(|b| Box::new(b)).parse(input)?;

    Ok((input, (test, body)))
}
