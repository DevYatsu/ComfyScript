use crate::parser::{
    ast::{identifier::Identifier, vars::VariableDeclarator, ASTNode},
    bool::parse_bool,
    numbers::parse_number,
    strings::parse_string,
    utils::alpha_not_reserved,
};
use nom::{
    branch::alt, character::complete::multispace0, combinator::map, error::VerboseError,
    multi::separated_list1, IResult, Parser,
};
use nom_supreme::tag::complete::tag;

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

pub fn parse_assignment(input: &str) -> IResult<&str, ASTNode, VerboseError<&str>> {
    let (input, keyword) = parse_variable_keyword(input)?;

    let (input, _) = multispace0(input)?;
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
    input: &str,
) -> IResult<&str, VariableDeclarator, VerboseError<&str>> {
    let (input, _) = multispace0(input)?;
    let (input, name) = alpha_not_reserved(input)?;

    let (input, _) = multispace0(input)?;

    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = alt((parse_bool, parse_number, parse_string))(input)?;

    let declarator = VariableDeclarator {
        id: Identifier {
            name: name.to_owned(),
        },
        init: value,
    };

    Ok((input, declarator))
}

fn parse_variable_keyword(i: &str) -> IResult<&str, VariableKeyword, VerboseError<&str>> {
    alt((
        map(tag("let"), |_| VariableKeyword::Let),
        map(tag("var"), |_| VariableKeyword::Var),
    ))
    .parse(i)
}
