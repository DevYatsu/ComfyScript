use crate::parser::{
    ast::{identifier::Identifier, vars::VariableDeclarator, ASTNode},
    bool::parse_bool,
};
use nom::{
    branch::alt,
    character::complete::{alphanumeric1, multispace0},
    combinator::map,
    error::{ContextError, VerboseError, VerboseErrorKind},
    multi::separated_list1,
    IResult, Parser,
};
use nom_supreme::tag::complete::tag;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum VariableKeyword {
    Var,
    Let,
}

impl VariableKeyword {
    pub fn equals_any(s: &str) -> bool {
        s == VariableKeyword::Let.to_string() || s == VariableKeyword::Var.to_string()
    }
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
    let (input, name) = alphanumeric1(input)?;

    if VariableKeyword::equals_any(name) {
        let e = VerboseError::add_context(
            input,
            "Invalid variable name!",
            VerboseError {
                errors: vec![(
                    "Invalid variable name!",
                    VerboseErrorKind::Context("Invalid variable name!"),
                )],
            },
        );
        return Err(nom::Err::Error(e));
    }

    let (input, _) = multispace0(input)?;

    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_bool(input)?;

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
