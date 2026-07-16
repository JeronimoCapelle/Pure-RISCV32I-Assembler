use std::num::ParseIntError;

use crate::utils::error::{AssemblerError, Stage::MathematicalBoundChecking};

pub fn i128_to_u8(x: i128) -> Result<u8, AssemblerError> {
    if x < (i128::from(u8::MIN)) || x > (i128::from(u8::MAX)) {
        return Err(AssemblerError::internal(MathematicalBoundChecking));
    }
    Ok(x as u8)
}

pub fn i128_to_i16(x: i128) -> Result<i16, AssemblerError> {
    if x < (i128::from(i16::MIN)) || x > (i128::from(i16::MAX)) {
        return Err(AssemblerError::internal(MathematicalBoundChecking));
    }
    Ok(x as i16)
}

pub fn i128_to_i32(x: i128) -> Result<i32, AssemblerError> {
    if x < (i128::from(i32::MIN)) || x > (i128::from(i32::MAX)) {
        return Err(AssemblerError::internal(MathematicalBoundChecking));
    }
    Ok(x as i32)
}

// ----

pub fn interpret_literal(value: &str) -> Result<i128, ParseIntError> {
    let _ = match value.strip_prefix("0b") {
        Some(a) => {
            let a = i128::from_str_radix(a, 2)?;
            return Ok(a);
        }
        None => 0,
    };

    let _ = match value.strip_prefix("0x") {
        Some(a) => {
            let a = i128::from_str_radix(a, 16)?;
            return Ok(a);
        }
        _ => 0,
    };

    let _ = match value.strip_prefix("0") {
        Some(a) => {
            if a.is_empty() {
                0
            } else {
                {
                    let a = i128::from_str_radix(a, 8)?;
                    return Ok(a);
                }
            }
        }
        _ => 0,
    };

    value.parse()
}
