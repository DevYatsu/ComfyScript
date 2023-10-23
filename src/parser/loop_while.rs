use super::{ast::ASTNode, expression::parse_expression, parse_block};
use nom::{
    character::complete::{multispace0, multispace1},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_while_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, _) = tag("while").complete().parse(input)?;
    let (input, _) = multispace1.cut().parse(input)?;

    let (input, test) = parse_expression.parse(input)?;
    let (input, _) = multispace0(input)?;
    println!("efef {:?}", test);
    let (input, body) = parse_block.cut().parse(input)?;

    let node = ASTNode::WhileStatement {
        test,
        body: Box::new(body),
    };

    Ok((input, node))
}
