#[derive(Debug, Clone, PartialEq)]
pub enum LiteralValue {
    Number(f32),
    Str(String),
    Boolean(bool),
    Nil,
}
