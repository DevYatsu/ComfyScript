use crate::expected;

use super::ast::{ASTNode, Expression};
use nom::{
    branch::alt, bytes::complete::take_until, character::complete::multispace0, multi::many0,
    sequence::preceded, IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn parse_comment_statement(input: &str) -> IResult<&str, ASTNode, ErrorTree<&str>> {
    let (input, comment) = parse_comment(input)?;

    let comment_statement = ASTNode::ExpressionStatement {
        expression: comment,
    };

    Ok((input, comment_statement))
}

pub fn parse_comment(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (input, comment) = alt((parse_line_comment, parse_multiline_comment))(input)?;

    Ok((input, comment))
}

pub fn jump_comments(input: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let (input, comments) = many0(preceded(
        multispace0,
        alt((parse_line_comment, parse_multiline_comment)),
    ))(input)?;
    let (input, _) = multispace0(input)?;

    let comments_str: String = comments.into_iter().map(|com| com.to_string()).collect();
    Ok((input, comments_str))
}

fn parse_line_comment(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (input, comment_opening) = tag("//")(input)?;
    let (input, comment_value) = take_until("\n")(input)?;
    let (input, comment_closing) = tag("\n")(input)?;

    let comment_expr = Expression::Comment {
        is_line: true,
        raw_value: comment_opening.to_string() + &comment_value + &comment_closing,
    };

    Ok((input, comment_expr))
}

fn parse_multiline_comment(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (input, comment_opening) = tag("/*")(input)?;
    let (input, comment_value) = take_until("*/")
        .context(expected!("*/"))
        .cut()
        .parse(input)?;
    let (input, comment_closing) = tag("*/")(input)?;

    let comment_expr = Expression::Comment {
        is_line: false,
        raw_value: comment_opening.to_owned() + &comment_value + &comment_closing,
    };

    Ok((input, comment_expr))
}
