use super::{
    ast::{ASTNode, Expression},
    Span,
};
use nom::{
    branch::alt, bytes::complete::take_until, character::complete::multispace0, multi::many0,
    sequence::preceded, IResult,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

pub fn parse_comment_statement(input: Span) -> IResult<Span, ASTNode, ErrorTree<Span>> {
    let (input, comment) = parse_comment(input)?;

    let comment_statement = ASTNode::ExpressionStatement {
        expression: comment,
    };

    Ok((input, comment_statement))
}

pub fn parse_comment(input: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (input, comment) = alt((parse_line_comment, parse_multiline_comment))(input)?;

    Ok((input, comment))
}

pub fn jump_comments(input: Span) -> IResult<Span, String, ErrorTree<Span>> {
    let (input, comments) = many0(preceded(
        multispace0,
        alt((parse_line_comment, parse_multiline_comment)),
    ))(input)?;
    let (input, _) = multispace0(input)?;

    let comments_str: String = comments.into_iter().map(|com| com.to_string()).collect();
    Ok((input, comments_str))
}

fn parse_line_comment(input: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (input, comment_opening) = tag("//")(input)?;
    let (input, comment_value) = take_until("\n")(input)?;
    let (input, comment_closing) = tag("\n")(input)?;

    let comment_expr = Expression::Comment {
        is_line: true,
        raw_value: comment_opening.to_string() + &comment_value + &comment_closing,
    };

    Ok((input, comment_expr))
}

fn parse_multiline_comment(input: Span) -> IResult<Span, Expression, ErrorTree<Span>> {
    let (input, comment_opening) = tag("/*")(input)?;
    let (input, comment_value) = take_until("*/")(input)?;
    let (input, comment_closing) = tag("*/")(input)?;

    let comment_expr = Expression::Comment {
        is_line: false,
        raw_value: comment_opening.fragment().to_string() + &comment_value + &comment_closing,
    };

    Ok((input, comment_expr))
}
