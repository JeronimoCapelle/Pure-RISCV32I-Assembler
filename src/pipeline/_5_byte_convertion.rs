//! Fifth step of the pipeline, Transforming encoded instructions into individual binary
/// Transforms the provided Vec<u32> into Vec<u8> for writing to binary file
pub(super) fn transform(input: Vec<u32>) -> Vec<u8> {
    let mut bytes: Vec<u8> = Vec::new();
    for word in input {
        bytes.extend_from_slice(&word.to_le_bytes());
    }

    bytes
}
