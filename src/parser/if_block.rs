use super::{
    ast::{ASTNode, Expression},
    expression::parse_expression,
    parse_block, Span,
};
use nom::{
    character::complete::{multispace0, multispace1},
    combinator::{map, opt},
    IResult,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

pub fn parse_if_statement(input: Span) -> IResult<Span, ASTNode, ErrorTree<Span>> {
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

fn parse_if_block(input: Span) -> IResult<Span, (Expression, Box<ASTNode>), ErrorTree<Span>> {
    let (input, _) = tag("if")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, test) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = map(parse_block, |b| Box::new(b))(input)?;

    Ok((input, (test, body)))
}
