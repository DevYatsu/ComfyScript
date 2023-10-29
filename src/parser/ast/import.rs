use std::fmt;

use super::identifier::Identifier;

#[derive(Debug, Clone, PartialEq)]
pub struct ImportSpecifier {
    pub local: Identifier, // name locally
    pub imported: Identifier,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ImportSource {
    pub value: String,
}

impl fmt::Display for ImportSpecifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.imported == self.local {
            write!(f, "{}", self.imported)
        } else {
            write!(f, "{} as {}", self.imported, self.local)
        }
    }
}
impl fmt::Display for ImportSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

impl ImportSource {
    pub fn console_print(&self) -> String {
        self.value[1..self.value.len() - 1].to_owned()
    }
}
