use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult};

use crate::parser::Span;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum AssignmentOperator {
    Equal,       // =
    PlusEqual,   // =+
    MinusEqual,  // =-
    TimesEqual,  // =*
    DivideEqual, // =/
    ModuloEqual, // =%
}

pub fn parse_assignment_operator(i: Span) -> IResult<Span, AssignmentOperator, VerboseError<Span>> {
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
        match t.fragment() {
            &"=" => AssignmentOperator::Equal,
            &"+=" => AssignmentOperator::PlusEqual,
            &"-=" => AssignmentOperator::MinusEqual,
            &"*=" => AssignmentOperator::TimesEqual,
            &"/=" => AssignmentOperator::DivideEqual,
            &"%=" => AssignmentOperator::ModuloEqual,
            _ => unreachable!(),
        },
    ))
}
