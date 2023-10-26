use std::fmt::Display;

use nom::{
    branch::alt,
    character::complete::alphanumeric1,
    combinator::{map, value},
    IResult, Parser,
};
use nom_supreme::{error::ErrorTree, tag::complete::tag, ParserExt};

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    String,
    Number,
    Bool,
    Nil,
    Object,
    Array,
    Range,
    Class(String),
}

pub fn parse_data_type(i: &str) -> IResult<&str, DataType, ErrorTree<&str>> {
    let (i, data_type) = alt((
        value(DataType::String, tag("String").complete()),
        value(DataType::Number, tag("Number").complete()),
        value(DataType::Array, tag("Array").complete()),
        value(DataType::Object, tag("Object").complete()),
        value(DataType::Range, tag("Range").complete()),
        value(DataType::Bool, tag("bool").complete()),
        value(DataType::Nil, tag("nil").complete()),
        map(alphanumeric1, |t: &str| DataType::Class(t.to_owned())),
    ))
    .parse(i)?;

    Ok((i, data_type))
}

impl Display for DataType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataType::String => write!(f, "String"),
            DataType::Number => write!(f, "Number"),
            DataType::Bool => write!(f, "bool"),
            DataType::Nil => write!(f, "nil"),
            DataType::Object => write!(f, "Object"),
            DataType::Array => write!(f, "Array"),
            DataType::Range => write!(f, "Range"),
            DataType::Class(value) => write!(f, "{}", value),
        }
    }
}
