use std::fmt;

use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult};

use crate::parser::Span;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Exponential,

    Equal,    // ==
    NotEqual, // !=
    Modulo,
    Greater,        // >
    GreaterOrEqual, // >=
    Smaller,        // <
    SmallerOrEqual, // <=

    And, // &&
    Or,  // ||
}

pub fn parse_binary_operator(i: Span) -> IResult<Span, BinaryOperator, VerboseError<Span>> {
    // one_of matches one of the characters we give it
    let (i, t) = alt((
        tag("+"),
        tag("-"),
        tag("*"),tag("**"),
        tag("/"),
        tag("%"),
        tag("=="),
        tag("!="),
        tag(">"),
        tag(">="),
        tag("<"),
        tag("<="),
        tag("&&"),
        tag("||"),
    ))(i)?;

    Ok((
        i,
        match t.fragment() {
            &"+" => BinaryOperator::Plus,
            &"-" => BinaryOperator::Minus,
            &"*" => BinaryOperator::Times,&"**" => BinaryOperator::Exponential,
            &"/" => BinaryOperator::Divide,
            &"%" => BinaryOperator::Modulo,
            &"==" => BinaryOperator::Equal,
            &"!=" => BinaryOperator::NotEqual,
            &">" => BinaryOperator::Greater,
            &">=" => BinaryOperator::GreaterOrEqual,
            &"<" => BinaryOperator::Smaller,
            &"<=" => BinaryOperator::SmallerOrEqual,
            &"&&" => BinaryOperator::And,
            &"||" => BinaryOperator::Or,
            _ => unreachable!(),
        },
    ))
}

impl fmt::Display for BinaryOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BinaryOperator::Plus => write!(f, "+"),
            BinaryOperator::Minus => write!(f, "-"),
            BinaryOperator::Times => write!(f, "*"),
            BinaryOperator::Exponential => write!(f, "**"),
            BinaryOperator::Divide => write!(f, "/"),
            BinaryOperator::Equal => write!(f, "=="),
            BinaryOperator::NotEqual => write!(f, "!="),
            BinaryOperator::Modulo => write!(f, "%"),
            BinaryOperator::Greater => write!(f, ">"),
            BinaryOperator::GreaterOrEqual => write!(f, ">="),
            BinaryOperator::Smaller => write!(f, "<"),
            BinaryOperator::SmallerOrEqual => write!(f, "<="),
            BinaryOperator::And => write!(f, "&&"),
            BinaryOperator::Or => write!(f, "||"),
        }
    }
}

impl BinaryOperator {
    pub fn get_precedence(&self) -> u8 {
        match self {
            BinaryOperator::Plus | BinaryOperator::Minus => 1,
            BinaryOperator::Times | BinaryOperator::Divide | BinaryOperator::Modulo => 2,
            BinaryOperator::Equal | BinaryOperator::NotEqual | BinaryOperator::Exponential => 3,
            BinaryOperator::Greater
            | BinaryOperator::GreaterOrEqual
            | BinaryOperator::Smaller
            | BinaryOperator::SmallerOrEqual
            | BinaryOperator::And
            | BinaryOperator::Or => 4,
        }
    }
}
