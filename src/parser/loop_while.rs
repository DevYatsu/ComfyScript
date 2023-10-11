use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    error::VerboseError,
    IResult,
};

use super::{ast::ASTNode, expression::parse_expression, parse_block, Span};

pub fn parse_while_statement(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, _) = tag("while")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, test) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;

    let (input, _) = tag("{")(input)?;

    let (input, body) = parse_block(input, "}")?;

    let node = ASTNode::WhileStatement {
        test,
        body: Box::new(body),
    };

    Ok((input, node))
}
