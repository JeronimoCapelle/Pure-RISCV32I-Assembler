use crate::auxiliar::error::AssemblerError;

mod auxiliar;
mod pipeline;

pub fn compile_string(input: &str) -> Result<Vec<u8>, AssemblerError> {
    pipeline::compile_string(input)
}
