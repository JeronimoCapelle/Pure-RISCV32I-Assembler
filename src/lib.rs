//! Public endpoint for the assembler as a library. Returns custom Error type with all info needed.

pub use crate::utils::error::AssemblerError;

mod pipeline;
mod utils;

pub fn assemble_string(input: &str) -> Result<Vec<u8>, AssemblerError> {
    match pipeline::compile_string(input) {
        Ok(a) => Ok(a),
        Err(mut a) => {
            a.fill_line(input);
            Err(a)
        }
    }
}
