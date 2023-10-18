pub fn expected_space() -> &'static str {
    "Expected a space"
}
pub fn expected_expression() -> &'static str {
    "Expected a valid expression"
}

#[macro_export]
macro_rules! expected_keyword {
    ($keyword:expr) => {
        concat!("Expected '{}' keyword", $keyword)
    };
}
#[macro_export]
macro_rules! expected_valid {
    ($x:expr) => {
        concat!("Expected valid {}", $x)
    };
}

#[macro_export]
macro_rules! expected {
    ($x:expr) => {
        concat!("Expected '{}'", $x)
    };
}
