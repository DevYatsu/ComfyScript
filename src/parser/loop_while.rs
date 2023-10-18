use super::{ast::ASTNode, expression::parse_expression, parse_block, Span};
use nom::{
    character::complete::{multispace0, multispace1},
    IResult,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

pub fn parse_while_statement(input: Span) -> IResult<Span, ASTNode, ErrorTree<Span>> {
    let (input, _) = tag("while")(input)?;
    let (input, _) = multispace1(input)?;

    let (input, test) = parse_expression(input)?;
    let (input, _) = multispace0(input)?;

    let (input, body) = parse_block(input)?;

    let node = ASTNode::WhileStatement {
        test,
        body: Box::new(body),
    };

    Ok((input, node))
}
