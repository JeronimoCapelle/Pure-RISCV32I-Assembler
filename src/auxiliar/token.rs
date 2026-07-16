#[derive(Debug, PartialEq, Eq, Clone)]
pub(crate) enum Token {
    Literal(String),
    Identifier(String),

    Comma,
    Colon,
    OpeningParenthesis,
    ClosingParenthesis,
    NewLine(usize),
}
