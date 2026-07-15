#[derive(Debug)]
pub struct TrackedError {
    pub kind: ParsingError,
    pub line: u32,
    pub file: &'static str, // Tracks the exact file (e.g., "src/structures.rs")
}

#[derive(Debug)]
pub enum ParsingError {
    BiggerValue,
    SmallerValue,
    OddValue,
    TexttoNumeric,
    NonExistentRegister,
    NonExistentMnemonic,
    WrongArgument,
    Tokenizer,
    NonLiteral,
    NonIdentifier,
    LabelTranslation,
    Empty,
}
