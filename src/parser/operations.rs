use nom::{
    branch::alt, bytes::complete::tag, character::complete::one_of, error::VerboseError, IResult,
};

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum Operator {
    Plus,
    Minus,
    Times,
    Divide,
    Equal, // ==
    Not,   // !=
    Modulo,
}

pub fn parse_operator(i: &str) -> IResult<&str, Operator, VerboseError<&str>> {
    // one_of matches one of the characters we give it
    let (i, t) = alt((
        tag("+"),
        tag("-"),
        tag("*"),
        tag("/"),
        tag("=="),
        tag("%"),
        tag("!="),
    ))(i)?;

    Ok((
        i,
        match t {
            "+" => Operator::Plus,
            "-" => Operator::Minus,
            "*" => Operator::Times,
            "/" => Operator::Divide,
            "%" => Operator::Modulo,
            "==" => Operator::Equal,
            "!=" => Operator::Not,

            _ => unreachable!(),
        },
    ))
}
