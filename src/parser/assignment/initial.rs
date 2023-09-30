use crate::parser::{
    ast::{identifier::Identifier, vars::VariableDeclarator, ASTNode},
    primitive_values::parse_primitive_value,
    utils::parse_identifier,
    Span,
};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{multispace0, multispace1},
    combinator::map,
    error::VerboseError,
    multi::separated_list1,
    IResult,
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum VariableKeyword {
    Var,
    Let,
}

impl ToString for VariableKeyword {
    fn to_string(&self) -> String {
        match self {
            VariableKeyword::Var => String::from("var"),
            VariableKeyword::Let => String::from("let"),
        }
    }
}

pub fn parse_assignment(input: Span) -> IResult<Span, ASTNode, VerboseError<Span>> {
    let (input, keyword) = parse_variable_keyword(input)?;

    let (input, _) = multispace1(input)?;
    let (input, declarations) = separated_list1(tag(","), parse_single_declaration)(input)?;

    let result = (
        input,
        ASTNode::VariableDeclaration {
            declarations,
            kind: keyword,
        },
    );

    Ok(result)
}

pub fn parse_single_declaration(
    input: Span,
) -> IResult<Span, VariableDeclarator, VerboseError<Span>> {
    let (input, _) = multispace0(input)?;
    let (input, name) = parse_identifier(input)?;

    let (input, _) = multispace0(input)?;

    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_primitive_value(input)?;

    let declarator = VariableDeclarator {
        id: Identifier { name },
        init: value,
    };

    Ok((input, declarator))
}

fn parse_variable_keyword(i: Span) -> IResult<Span, VariableKeyword, VerboseError<Span>> {
    alt((
        map(tag("let"), |_| VariableKeyword::Let),
        map(tag("var"), |_| VariableKeyword::Var),
    ))(i)
}
