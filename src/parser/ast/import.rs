use super::identifier::Identifier;

#[derive(Debug, Clone)]
pub struct ImportSpecifier {
    pub local: Identifier,
    pub imported: Identifier, // name locally
}
