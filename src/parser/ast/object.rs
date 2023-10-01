use super::{identifier::Identifier, Expression};

#[derive(Debug, Clone)]
pub struct Property {
    pub method: bool,
    pub shorthand: bool,
    pub computed: bool,
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
