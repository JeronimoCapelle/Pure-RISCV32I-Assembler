use core::fmt;
use std::panic::Location;

use crate::auxiliar::{error::Responsible::Internal, token::Token};

// ----

#[derive(Debug)]
pub struct AssemblerError {
    rust_location: &'static Location<'static>,
    assembly_stage: Stage,
    input_line_number: usize,
    who: Responsible,
}

impl AssemblerError {
    #[track_caller]
    pub const fn new(assembly_stage: Stage, input_line_number: usize) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            input_line_number,
            who: Responsible::User,
        }
    }
    pub const fn internal(assembly_stage: Stage) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            who: Internal,
            input_line_number: 0,
        }
    }
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[Error encountered, Fault:{:?}, Stage:{:?}, Location:{}, line:{} ]",
            self.who, self.assembly_stage, self.rust_location, self.input_line_number
        )
    }
}

#[derive(Debug)]
pub enum Responsible {
    Internal,
    User,
}

#[derive(Debug)]
pub enum Stage {
    Tokenizer,
    SymbolCollection,
    Syntax(SyntaxError),
    MathematicalBoundChecking,
}

#[derive(Debug)]
pub enum SyntaxError {
    BiggerValue(i128, i128),  //expected, recieved
    SmallerValue(i128, i128), //expected, recieved
    OddValue(Token),
    TexttoNumeric(Token),
    NonExistentRegister(Token),
    NonExistentMnemonic(Token),
    WrongArguments,
    InvalidStartingWord(Token),
    InvalidToken(Token),
    Translation(Token),
    Internal,
    Empty,
}
