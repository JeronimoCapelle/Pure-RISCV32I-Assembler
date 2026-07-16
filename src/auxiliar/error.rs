use core::fmt;
use std::{error::Error, panic::Location};

use crate::auxiliar::{error::Responsible::Internal, token::Token};

// ----

#[derive(Debug)]
pub struct AssemblerError {
    rust_location: &'static Location<'static>,
    assembly_stage: Stage,
    input_line_number: usize,
    input_line: String,
    who: Responsible,
}

impl Error for AssemblerError {}

impl AssemblerError {
    #[track_caller]
    pub fn new(assembly_stage: Stage, input_line_number: usize) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            input_line_number,
            who: Responsible::User,
            input_line: "".to_string(),
        }
    }
    pub fn internal(assembly_stage: Stage) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            who: Internal,
            input_line_number: 0,
            input_line: "".to_string(),
        }
    }
    pub fn fill_line(&mut self, file: &str) {
        self.input_line = file
            .lines()
            .nth(self.input_line_number - 1)
            .unwrap()
            .to_string();
    }
}

impl fmt::Display for AssemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
[  Error encountered: {:?} ] ({:?})
 
 --> line:{}
    | 
    |  {} 
    |

[ Location:{} ]
",
            self.assembly_stage,
            self.who,
            self.input_line_number,
            self.input_line,
            self.rust_location
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
