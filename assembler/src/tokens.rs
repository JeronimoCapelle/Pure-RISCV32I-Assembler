///Tokens for the tokenization step of the assembler, these are very general
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Token {
    Literal(String),
    Identifier(String),

    Coma,
    Colon,
    OpeningParenthesis,
    ClosingParenthesis,
    NewLine,
}
