use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum RangeType {
    Dot,      // ..
    DotEqual, // ..=
}

impl fmt::Display for RangeType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RangeType::Dot => write!(f, ".."),
            RangeType::DotEqual => write!(f, "..="),
        }
    }
}
