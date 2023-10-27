use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f32),
    Str(String),
    Boolean(bool),
    Nil,
}

impl Display for LiteralValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LiteralValue::Number(num) => write!(f, "{}", num),
            LiteralValue::Str(string) => {
                write!(f, "{}", string)
            }
            LiteralValue::Boolean(b) => write!(f, "{}", b),
            LiteralValue::Nil => write!(f, "nil"),
        }
    }
}

impl LiteralValue {
    pub fn is_falsy(&self) -> bool {
        match self {
            LiteralValue::Number(num) => num == &0.0,
            LiteralValue::Str(s) => s.is_empty(),
            LiteralValue::Boolean(b) => *b,
            LiteralValue::Nil => true,
        }
    }
}
