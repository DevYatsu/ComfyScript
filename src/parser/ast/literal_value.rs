use crate::parser::expression::strings::StringFragment;

#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f32),
    Str(Vec<StringFragment>),
    Boolean(bool),
    Nil,
}
