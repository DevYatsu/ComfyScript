use std::fmt;

use nom::{branch::alt, IResult};
use nom_supreme::{error::ErrorTree, tag::complete::tag};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AssignmentOperator {
    Equal,       // =
    PlusEqual,   // =+
    MinusEqual,  // =-
    TimesEqual,  // =*
    DivideEqual, // =/
    ModuloEqual, // =%
}

pub fn parse_assignment_operator(i: &str) -> IResult<&str, AssignmentOperator, ErrorTree<&str>> {
    // one_of matches one of the characters we give it
    let (i, t) = alt((
        tag("="),
        tag("+="),
        tag("-="),
        tag("*="),
        tag("/="),
        tag("%="),
    ))(i)?;

    Ok((
        i,
        match t {
            "=" => AssignmentOperator::Equal,
            "+=" => AssignmentOperator::PlusEqual,
            "-=" => AssignmentOperator::MinusEqual,
            "*=" => AssignmentOperator::TimesEqual,
            "/=" => AssignmentOperator::DivideEqual,
            "%=" => AssignmentOperator::ModuloEqual,
            _ => unreachable!(),
        },
    ))
}

impl fmt::Display for AssignmentOperator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AssignmentOperator::Equal => write!(f, "="),
            AssignmentOperator::PlusEqual => write!(f, "+="),
            AssignmentOperator::MinusEqual => write!(f, "-="),
            AssignmentOperator::TimesEqual => write!(f, "*="),
            AssignmentOperator::DivideEqual => write!(f, "/="),
            AssignmentOperator::ModuloEqual => write!(f, "%="),
        }
    }
}
