use crate::{auxiliar::error::AssemblerError, pipeline::_5_byte_convertion::transform};

mod _1_lexical_analysis;
mod _2_symbol_resolution;
mod _3_syntax_analysis;
mod _4_code_generation;
mod _5_byte_convertion;

pub fn compile_string(input: &str) -> Result<Vec<u8>, AssemblerError> {
    let tokens = _1_lexical_analysis::tokenize(input)?;
    let (labels, stripped_tokens) = _2_symbol_resolution::collect_symbols(&tokens)?;
    let statements = _3_syntax_analysis::parse(&stripped_tokens, &labels)?;
    let packed = _4_code_generation::assemble(statements);
    Ok(transform(packed))
}
