use crate::parser::{bool::parse_bool, builtins::Atom};
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, multispace0},
    combinator::map,
    error::{ContextError, VerboseError, VerboseErrorKind},
    IResult, Parser,
};

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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Assignment {
    Constant { name: String, value: Atom },
    Mutable { name: String, value: Atom },
}

impl Assignment {
    pub fn new(keyword: VariableKeyword, name: String, value: Atom) -> Self {
        match keyword {
            VariableKeyword::Var => Assignment::Mutable { name, value },
            VariableKeyword::Let => Assignment::Constant { name, value },
        }
    }
}

pub fn parse_assignment(input: &str) -> IResult<&str, Assignment, VerboseError<&str>> {
    let (input, keyword) = parse_variable_keyword(input)?;

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
    };

    let (input, _) = multispace0(input)?;

    let (input, _) = tag("=")(input)?;
    let (input, _) = multispace0(input)?;
    let (input, value) = parse_bool(input)?;

    let result = (input, Assignment::new(keyword, name.to_owned(), value));

    Ok(result)
}

fn parse_variable_keyword(i: &str) -> IResult<&str, VariableKeyword, VerboseError<&str>> {
    alt((
        map(tag("let"), |_| VariableKeyword::Let),
        map(tag("var"), |_| VariableKeyword::Var),
    ))
    .parse(i)
}
