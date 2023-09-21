use super::{identifier::Identifier, Expression};

#[derive(Debug, Clone)]
pub struct VariableDeclarator {
    pub id: Identifier,
    pub init: Expression,
}
