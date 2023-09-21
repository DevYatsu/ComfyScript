use super::operations::Operation;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Atom {
    Num(i32),
    Keyword(String),
    Boolean(bool),
    Operation(Operation),
    Str(String),
}
