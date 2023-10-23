use std::fmt;

use super::{identifier::Identifier, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: Expression,
}
impl fmt::Display for VariableDeclarator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}={}", self.id, self.init)
    }
}
