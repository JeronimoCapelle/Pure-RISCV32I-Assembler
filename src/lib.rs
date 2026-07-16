use crate::auxiliar::error::AssemblerError;

pub mod auxiliar;
mod pipeline;

pub fn compile_string(input: &str) -> Result<Vec<u8>, AssemblerError> {
    match pipeline::compile_string(input) {
        Ok(a) => Ok(a),
        Err(mut a) => {
            a.fill_line(&input);
            Err(a)
        }
    }
}
