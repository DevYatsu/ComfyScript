use std::fmt;

use nom::{branch::alt, IResult};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

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
    let (i, assignment_op) = alt((
        tag("=").value(AssignmentOperator::Equal),
        tag("+=").complete().value(AssignmentOperator::PlusEqual),
        tag("-=").complete().value(AssignmentOperator::MinusEqual),
        tag("*=").complete().value(AssignmentOperator::TimesEqual),
        tag("/=").complete().value(AssignmentOperator::DivideEqual),
        tag("%=").complete().value(AssignmentOperator::ModuloEqual),
    ))(i)?;

    Ok((i, assignment_op))
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
