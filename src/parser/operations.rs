use nom::{character::complete::one_of, error::VerboseError, IResult};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Equal, // ==
    Not,   // !=
}

pub fn parse_operator(i: &str) -> IResult<&str, Operator, VerboseError<&str>> {
    // one_of matches one of the characters we give it
    let (i, t) = one_of("+-*/=")(i)?;

    Ok((
        i,
        match t {
            '+' => Operator::Plus,
            '-' => Operator::Minus,
            '*' => Operator::Times,
            '/' => Operator::Divide,
            '=' => Operator::Equal,
            _ => unreachable!(),
        },
    ))
}
