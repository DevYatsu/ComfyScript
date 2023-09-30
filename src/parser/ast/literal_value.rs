#[derive(Debug, Clone)]
pub enum LiteralValue {
    Number(f32),
    Str(String),
    Boolean(bool),
    Nil
}
