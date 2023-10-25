use std::fmt::Display;

use crate::parser::expression::strings::StringFragment;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f32),
    Str(Vec<StringFragment>),
    Boolean(bool),
    Nil,
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Number(num) => write!(f, "{}", num),
            LiteralValue::Str(s) => {
                for fragment in s {
                    write!(f, "{}", fragment)?;
                }

                write!(f, "")
            }
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}
