use crate::structures::{ParsingError::Empty, TrackedError};

pub fn transform(input: Vec<u32>) -> Result<Vec<u8>, TrackedError> {
    if input.is_empty() {
        return Err(TrackedError {
            kind: Empty,
            line: line!(),
            file: file!(),
        });
    }
    let mut bytes: Vec<u8> = Vec::new();
    for word in input {
        bytes.push((word & 0b1111_1111) as u8);
        bytes.push(((word >> 8) & 0b1111_1111) as u8);
        bytes.push(((word >> 16) & 0b1111_1111) as u8);
        bytes.push(((word >> 24) & 0b1111_1111) as u8);
    }

    Ok(bytes)
}
