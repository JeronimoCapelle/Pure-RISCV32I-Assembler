//! pipeline manager which is in charge of executing each step of the assembly and return an error if encountered.

use crate::utils::error::AssemblerError;

mod _1_lexical_analysis;
mod _2_symbol_resolution;
mod _3_syntax_analysis;
mod _4_code_generation;
mod _5_byte_conversion;

/// Manages the assembly pipeline from program string to binary, returns user and programmer errors.
pub fn compile_string(input: &str) -> Result<Vec<u8>, AssemblerError> {
    let extracted_tokens = _1_lexical_analysis::tokenize(input)?;
    let (labels_map, stripped_tokens) = _2_symbol_resolution::collect_symbols(&extracted_tokens)?;
    let instructions = _3_syntax_analysis::parse(&stripped_tokens, &labels_map)?;
    let assembled_binary = _4_code_generation::assemble(instructions);
    Ok(_5_byte_conversion::transform(assembled_binary))
}
