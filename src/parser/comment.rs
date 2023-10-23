use super::ast::Expression;
use nom::{
    branch::alt,
    bytes::complete::{take, take_until},
    character::complete::multispace0,
    multi::many0,
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

pub fn jump_comments(input: &str) -> IResult<&str, String, ErrorTree<&str>> {
    let (input, _) = multispace0(input)?;
    let (input, comments) =
        many0(alt((parse_line_comment, parse_multiline_comment)).preceded_by(multispace0))(input)?;
    let (input, _) = multispace0(input)?;

    let comments_str: String = comments.into_iter().map(|com| com.to_string()).collect();
    Ok((input, comments_str))
}
pub fn multispace0comments(input: &str) -> IResult<&str, String, ErrorTree<&str>> {
    // to update to parse both multispaces and comments
    let (input, _) = multispace0(input)?;

    let (input, comments) = many0(
        alt((
            tag("//").complete().and_then(parse_line_comment),
            tag("/*").complete().and_then(parse_multiline_comment),
        ))
        .preceded_by(multispace0),
    )(input)?;

    let (input, _) = multispace0(input)?;

    let comments_str: String = comments.into_iter().map(|com| com.to_string()).collect();
    Ok((input, comments_str))
}

pub fn parse_line_comment(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    if !input.contains("\n") {
        // if we arrive at the end of input

        let (input, comment_value) = take(input.len())(input)?;

        let comment_expr = Expression::Comment {
            is_line: true,
            raw_value: "//".to_string() + &comment_value,
        };
        return Ok((input, comment_expr));
    }

    let (input, comment_value) = take_until("\n")(input)?;

    let (input, comment_closing) = tag("\n").complete().parse(input)?;

    let comment_expr = Expression::Comment {
        is_line: true,
        raw_value: "//".to_string() + &comment_value + &comment_closing,
    };

    Ok((input, comment_expr))
}

pub fn parse_multiline_comment(input: &str) -> IResult<&str, Expression, ErrorTree<&str>> {
    let (input, comment_value) = take_until("*/").cut().parse(input)?;
    let (input, comment_closing) = tag("*/").parse(input)?;

    let comment_expr = Expression::Comment {
        is_line: false,
        raw_value: "/*".to_owned() + &comment_value + &comment_closing,
    };

    Ok((input, comment_expr))
}
