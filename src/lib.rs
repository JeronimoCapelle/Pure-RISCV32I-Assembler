use crate::structures::error::TrackedError;

mod pipeline;
mod structures;

pub fn compile_string(input: &str) -> Result<Vec<u8>, TrackedError> {
    pipeline::compile_string(input)
}
