use nom::{branch::alt, bytes::complete::tag, error::VerboseError, IResult};

use super::Span;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
    Equal, // ==
    Not,   // !=
    Modulo,
    Greater,        // >
    GreaterOrEqual, // >=
    Smaller,        // <
    SmallerOrEqual, // <=
}

pub fn parse_binary_operator(i: Span) -> IResult<Span, BinaryOperator, VerboseError<Span>> {
    // one_of matches one of the characters we give it
    let (i, t) = alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("%"),
        tag("=="),
        tag("!="),
        tag(">"),
        tag(">="),
        tag("<"),
        tag("<="),
    ))(i)?;

    Ok((
        i,
        match t.fragment() {
            &"+" => BinaryOperator::Plus,
            &"-" => BinaryOperator::Minus,
            &"*" => BinaryOperator::Times,
            &"/" => BinaryOperator::Divide,
            &"%" => BinaryOperator::Modulo,
            &"==" => BinaryOperator::Equal,
            &"!=" => BinaryOperator::Not,
            &">" => BinaryOperator::Greater,
            &">=" => BinaryOperator::GreaterOrEqual,
            &"<" => BinaryOperator::Smaller,
            &"<=" => BinaryOperator::SmallerOrEqual,

            _ => unreachable!(),
        },
    ))
}

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
    let (i, t) = alt((tag("+="), tag("-="), tag("*="), tag("/="), tag("%=")))(i)?;

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
