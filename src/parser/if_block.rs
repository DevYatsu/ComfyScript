use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::opt,
    error::VerboseError,
    IResult,
};

use super::{ast::ASTNode, expression::parse_expression, parse_block, Span};

pub fn parse_if_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, _) = tag("if")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, test) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("{")(input)?;

    let (input, body) = parse_block(input, "}")?;

    let (else_input, _) = multispace0(input)?;
    let (else_input, else_word) = opt(tag("else"))(else_input)?;

    if else_word.is_none() {
        let node = ASTNode::IfStatement {
            test,
            body: Box::new(body),
            alternate: None,
        };

        return Ok((input, node));
    }

    let (else_input, _) = multispace0(else_input)?;
    let (_, other_if) = opt(tag("if"))(else_input)?;

    if other_if.is_none() {
        let (else_input, _) = tag("{")(else_input)?;

        let (else_input, alternate) = parse_block(else_input, "}")?;

        let node = ASTNode::IfStatement {
            test,
            body: Box::new(body),
            alternate: Some(Box::new(alternate)),
        };

        return Ok((else_input, node));
    }

    let (other_if_input, statement) = parse_if_statement(else_input)?;

    let node = ASTNode::IfStatement {
        test,
        body: Box::new(body),
        alternate: Some(Box::new(statement)),
    };

    Ok((other_if_input, node))
}
