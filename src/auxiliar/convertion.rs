// mathematical proof that an i16 can be converted to u32 losslessly
// pub fn i16_to_u32(x: i16) -> Result<u32, AssemblerError> {
//     if x < 0 {
//         return Err(AssemblerError::internal(MathematicalProof));
//     };
//     Ok(x as u32)
// }

//mathematical proof that an i32 can be converted to u32 losslessly
// pub fn i32_to_u32(x: i32) -> Result<u32, AssemblerError> {
//     if x < 0 {
//         return Err(AssemblerError::internal(MathematicalProof));
//     };
//     Ok(x as u32)
// }

/// mathematical proof that an i16 can be converted to u32 losslessly
pub fn i16_to_u32(x: i16) -> u32 {
    assert!(x >= 0);

    x as u32
}

/// mathematical proof that an i32 can be converted to u32 losslessly
pub fn i32_to_u32(x: i32) -> u32 {
    assert!(x >= 0);

    x as u32
}
