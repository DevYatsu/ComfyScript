use std::fmt;

use crate::parser::data_type::DataType;

use super::{identifier::Identifier, Expression};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: Expression,
    pub var_type: Option<DataType>,
}
impl fmt::Display for VariableDeclarator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)?;

        if let Some(t) = &self.var_type {
            write!(f, ":{}", t)?;
        }

        write!(f, "={}", self.init)
    }
}
