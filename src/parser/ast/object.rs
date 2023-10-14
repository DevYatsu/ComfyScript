use std::fmt;

use super::{identifier::Identifier, Expression};

#[derive(Debug, Clone)]
pub struct Property {
    pub is_method: bool,
    pub shorthand: bool,
    pub key: Identifier,
    pub value: Expression,
    pub kind: PropertyKind,
}

#[derive(Debug, Clone)]
pub enum PropertyKind {
    Init,
    Get,
    Set,
}
impl fmt::Display for Property {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", self.key, self.value)
    }
}
