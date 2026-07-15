#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Literal(String),
    Identifier(String),

    Comma,
    Colon,
    OpeningParenthesis,
    ClosingParenthesis,
    NewLine(usize),
}
