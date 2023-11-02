use std::fmt;

use nom::{branch::alt, IResult};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

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

pub fn parse_binary_operator(i: &str) -> IResult<&str, BinaryOperator, ErrorTree<&str>> {
    let (i, operator) = alt((
        tag("+").value(BinaryOperator::Plus),
        tag("-").value(BinaryOperator::Minus),
        tag("**").complete().value(BinaryOperator::Exponential),
        tag("*").value(BinaryOperator::Times),
        tag("/").value(BinaryOperator::Divide),
        tag("%").value(BinaryOperator::Modulo),
        tag("==").complete().value(BinaryOperator::Equal),
        tag("!=").complete().value(BinaryOperator::NotEqual),
        tag(">=").complete().value(BinaryOperator::GreaterOrEqual),
        tag(">").value(BinaryOperator::Greater),
        tag("<=").complete().value(BinaryOperator::SmallerOrEqual),
        tag("<").value(BinaryOperator::Smaller),
        tag("&&").complete().value(BinaryOperator::And),
        tag("||").complete().value(BinaryOperator::Or),
    ))(i)?;

    Ok((i, operator))
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
            BinaryOperator::And | BinaryOperator::Or => 0,
            BinaryOperator::Plus | BinaryOperator::Minus => 1,
            BinaryOperator::Times | BinaryOperator::Divide | BinaryOperator::Modulo => 2,
            BinaryOperator::Exponential => 3,
            BinaryOperator::Equal
            | BinaryOperator::NotEqual
            | BinaryOperator::Greater
            | BinaryOperator::GreaterOrEqual
            | BinaryOperator::Smaller
            | BinaryOperator::SmallerOrEqual => 4,
        }
    }
}
