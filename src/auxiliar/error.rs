use core::fmt;
use std::{error::Error, fmt::Debug, panic::Location};

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
    pub(crate) fn new(assembly_stage: Stage, input_line_number: usize) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            input_line_number,
            who: Responsible::User,
            input_line: "".to_string(),
        }
    }
    pub(crate) fn internal(assembly_stage: Stage) -> Self {
        Self {
            rust_location: Location::caller(),
            assembly_stage,
            who: Internal,
            input_line_number: 0,
            input_line: "".to_string(),
        }
    }
    pub(crate) fn fill_line(&mut self, file: &str) {
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
\t[ Error encountered: {:?} ] ({:?})
 
 --> line:{}
  | 
  |  {} 
  |

\t[ Location:{} ]
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
pub(crate) enum Responsible {
    Internal,
    User,
}

#[derive(Debug)]
pub(crate) enum Stage {
    Tokenizer,
    SymbolCollection,
    Syntax(SyntaxError),
    MathematicalBoundChecking,
}

pub(crate) enum SyntaxError {
    BiggerValue(i128, i128),  //expected, recieved
    SmallerValue(i128, i128), //expected, recieved
    OddValue(i128),
    TexttoNumeric(String),
    NonExistentRegister(String),
    NonExistentMnemonic(String),
    WrongArguments,
    InvalidToken(Token),
    Translation(String),
    Internal,
}

impl Debug for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::BiggerValue(max, value) => write!(
                f,
                "Value provided {{ {value} }} is bigger than max {{ {max} }}.",
            ),
            SyntaxError::SmallerValue(min, value) => write!(
                f,
                "Value provided {{ {value} }} is smaller than min {{ {min} }}."
            ),
            SyntaxError::OddValue(offset) => write!(f, "Offset {{{offset}}} is an Odd value."),
            SyntaxError::TexttoNumeric(string) => write!(
                f,
                "Could not convert {{{string}}} to a number, make sure it is one."
            ),
            SyntaxError::NonExistentRegister(string) => {
                write!(f, "Could not identify {{{string}}} register")
            }
            SyntaxError::NonExistentMnemonic(string) => write!(
                f,
                "Could not identify {{{string}}} mnemonic, it might be mispelled or not yet implemented"
            ),
            SyntaxError::Translation(string) => {
                write!(f, "Couldnt find reference to label {{{string}}}")
            }
            SyntaxError::Internal => write!(
                f,
                "something went very wrong, you are not supposed to be seeing this..."
            ),
            SyntaxError::WrongArguments => {
                write!(f, "Invalid arguments were passed to this instruction")
            }
            SyntaxError::InvalidToken(token) => write!(f, "Did not expect the token {{{token:?}}}"),
        }
    }
}
