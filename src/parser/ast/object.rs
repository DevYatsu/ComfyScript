use super::{identifier::Identifier, Expression};

#[derive(Debug, Clone)]
pub struct Property {
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
    pub key: Identifier,
    pub value: Expression,
    pub kind: String,
}
