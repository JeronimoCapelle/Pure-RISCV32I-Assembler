use std::collections::HashMap;

use crate::auxiliar::{
    convertion::{i16_to_u32, i32_to_u32},
    error::SyntaxError::{
        self, BiggerValue, InvalidWord, NonExistentRegister, OddValue, SmallerValue, TexttoNumeric,
        Translation,
    },
    token::Token,
};

#[derive(PartialEq, Eq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub enum Instruction {
    ADDI(IType),
    ADD(RType),
    SUB(RType),
    BNE(BType),
    BEQ(BType),
    BLT(BType),
    BGE(BType),
    JAL(JType),
    JALR(ITypeJump),
    LW(ITypeMemory),
    SW(STypeMemory),
    LB(ITypeMemory),
    SB(STypeMemory),
    SLLI(ITypeShifts),
    SRLI(ITypeShifts),
    AND(RType),
    OR(RType),
    XOR(RType),
    ANDI(IType),
    ORI(IType),
    XORI(IType),
}

//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
pub struct RType {
    pub destination: Register,
    pub first_source: Register,
    pub second_source: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct IType {
    pub destination: Register,
    pub source: Register,
    pub immediate: Immediate,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeShifts {
    pub destination: Register,
    pub source: Register,
    pub shamt: Shamt,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeMemory {
    pub destination: Register,
    pub offset: Offset,
    pub base_address: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct STypeMemory {
    pub source: Register,
    pub offset: Offset,
    pub base_address: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub struct BType {
    pub first_source: Register,
    pub second_source: Register,
    pub label: Label,
}
#[derive(PartialEq, Eq, Debug)]
pub struct JType {
    pub destination: Register,
    pub big_label: BigLabel,
}
#[derive(PartialEq, Eq, Debug)]
pub struct ITypeJump {
    pub destination: Register,
    pub offset: Offset,
    pub target_address: Register,
}

//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
pub struct Immediate(i16); // 12-bit signed integer (range: -2048 to 2047). Limit artificially

impl Immediate {
    pub fn new(token: &Token) -> Result<Immediate, SyntaxError> {
        let max = 2047;
        let min = -2048;

        let Token::Literal(value) = token else {
            return Err(InvalidWord(token.clone()));
        };

        let numeric: i16;

        if value.starts_with("0b") {
            numeric = match i16::from_str_radix(value.strip_prefix("0b").unwrap().trim(), 2) {
                Ok(a) => a,
                Err(_) => return Err(TexttoNumeric(token.clone())),
            }
        } else if value.starts_with("0x") {
            numeric = match i16::from_str_radix(value.strip_prefix("0x").unwrap().trim(), 16) {
                Ok(a) => a,
                Err(_) => return Err(TexttoNumeric(token.clone())),
            }
        } else {
            numeric = match value.parse() {
                Ok(a) => a,
                Err(_) => return Err(TexttoNumeric(token.clone())),
            }
        }

        if numeric < min {
            return Err(SmallerValue(min as i32, numeric as i32));
        } else if numeric > max {
            return Err(BiggerValue(max as i32, numeric as i32));
        }
        Ok(Immediate(numeric))
    }

    pub fn encode(&self) -> u32 {
        if self.0 >= 0 {
            i16_to_u32(self.0)
        } else {
            i16_to_u32(self.0 + 2048) | 2048
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Shamt(u8); //5-bit unsigned integer (range: 0 to 31 for 32-bit registers). Limit artificially

impl Shamt {
    pub fn new(token: &Token) -> Result<Shamt, SyntaxError> {
        let max = 31;

        let Token::Literal(value) = token else {
            return Err(InvalidWord(token.clone()));
        };

        let numeric: u8;

        if value.starts_with("0b") {
            numeric = match u8::from_str_radix(value.strip_prefix("0b").unwrap().trim(), 2) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TexttoNumeric(token.clone()));
                }
            }
        } else if value.starts_with("0x") {
            numeric = match u8::from_str_radix(value.strip_prefix("0x").unwrap().trim(), 16) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TexttoNumeric(token.clone()));
                }
            }
        } else {
            numeric = match value.parse() {
                Ok(a) => a,
                Err(_) => {
                    return Err(TexttoNumeric(token.clone()));
                }
            }
        }

        if numeric > max {
            return Err(BiggerValue(max as i32, numeric as i32));
        }
        Ok(Shamt(numeric))
    }
    pub fn encode(&self) -> u32 {
        u32::from(self.0)
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Offset(i16); //12-bit signed immediate offset (range: -2048 to 2047 bytes). Limit artificially

impl Offset {
    pub fn new(token: &Token) -> Result<Offset, SyntaxError> {
        let max = 2047;
        let min = -2048;
        let Token::Literal(value) = token else {
            return Err(InvalidWord(token.clone()));
        };

        let numeric: i16;

        if value.starts_with("0b") {
            numeric = match i16::from_str_radix(value.strip_prefix("0b").unwrap().trim(), 2) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TexttoNumeric(token.clone()));
                }
            }
        } else if value.starts_with("0x") {
            numeric = match i16::from_str_radix(value.strip_prefix("0x").unwrap().trim(), 16) {
                Ok(a) => a,
                Err(_) => {
                    return Err(TexttoNumeric(token.clone()));
                }
            }
        } else {
            numeric = match value.parse() {
                Ok(a) => a,
                Err(_) => {
                    return Err(TexttoNumeric(token.clone()));
                }
            }
        }

        if numeric < min {
            return Err(SmallerValue(min as i32, numeric as i32));
        }

        if numeric > max {
            return Err(BiggerValue(max as i32, numeric as i32));
        }
        Ok(Offset(numeric))
    }
    pub fn encode(&self) -> u32 {
        if self.0 >= 0 {
            i16_to_u32(self.0)
        } else {
            i16_to_u32(self.0 + 2048) | 2048
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct Label(i16); //12-bit signed PC-relative offset. limit artificially. multiple of 2 bytes

impl Label {
    pub fn new(
        token: &Token,
        symbol_table: &HashMap<String, usize>,
        current_pc: usize,
    ) -> Result<Label, SyntaxError> {
        let min = -4096;
        let max = 4094;

        let Token::Identifier(value) = token else {
            return Err(InvalidWord(token.clone()));
        };
        if !symbol_table.contains_key(value) {
            return Err(Translation(token.clone()));
        }
        let offset: i128 = i128::try_from(*symbol_table.get(value).unwrap()).unwrap()
            - i128::try_from(current_pc).unwrap();
        if offset % 2 != 0 {
            return Err(OddValue(token.clone()));
        }
        if offset < min {
            return Err(SmallerValue(min as i32, offset as i32));
        }

        if offset > max {
            return Err(BiggerValue(max as i32, offset as i32));
        }

        Ok(Label(offset.try_into().unwrap()))
    }
    pub fn encode(&self) -> u32 {
        if self.0 >= 0 {
            i16_to_u32(self.0)
        } else {
            i16_to_u32(self.0 + 4096) | 4096
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub struct BigLabel(i32); //20-bit signed PC-relative offset. Limit artificially. multiple of 2 bytes

impl BigLabel {
    pub fn new(
        token: &Token,
        symbol_table: &HashMap<String, usize>,
        current_pc: usize,
    ) -> Result<BigLabel, SyntaxError> {
        let min = -1_048_576;
        let max = 1_048_574;

        let Token::Identifier(value) = token else {
            return Err(InvalidWord(token.clone()));
        };
        if !symbol_table.contains_key(value) {
            return Err(Translation(token.clone()));
        }
        let offset = (i128::try_from(*symbol_table.get(value).unwrap()).unwrap()
            - i128::try_from(current_pc).unwrap())
        .try_into()
        .unwrap();

        if offset % 2 != 0 {
            return Err(OddValue(token.clone()));
        }
        if offset < min {
            return Err(SmallerValue(min, offset));
        }

        if offset > max {
            return Err(BiggerValue(max, offset));
        }

        Ok(BigLabel(offset))
    }

    pub fn encode(&self) -> u32 {
        if self.0 >= 0 {
            i32_to_u32(self.0)
        } else {
            i32_to_u32(self.0 + 1_048_576) | 1_048_576
        }
    }
}
//--------------------------------------------------
#[derive(PartialEq, Eq, Debug)]
pub enum Register {
    X0,
    X1,
    X2,
    X3,
    X4,
    X5,
    X6,
    X7,
    X8,
    X9,
    X10,
    X11,
    X12,
    X13,
    X14,
    X15,
    X16,
    X17,
    X18,
    X19,
    X20,
    X21,
    X22,
    X23,
    X24,
    X25,
    X26,
    X27,
    X28,
    X29,
    X30,
    X31,
}

impl Register {
    pub fn new(token: &Token) -> Result<Register, SyntaxError> {
        let Token::Identifier(name) = token else {
            return Err(InvalidWord(token.clone()));
        };
        Ok(match name.as_str() {
            "x0" | "zero" => Register::X0,

            "x1" | "ra" => Register::X1,
            "x2" | "sp" => Register::X2,
            "x3" | "gp" => Register::X3,
            "x4" | "tp" => Register::X4,
            //---
            "x5" | "t0" => Register::X5,
            "x6" | "t1" => Register::X6,
            "x7" | "t2" => Register::X7,
            //---
            "x8" | "fp" | "s0" => Register::X8,
            "x9" | "s1" => Register::X9,
            "x10" | "a0" => Register::X10,
            //---
            "x11" | "a1" => Register::X11,
            "x12" | "a2" => Register::X12,
            "x13" | "a3" => Register::X13,
            "x14" | "a4" => Register::X14,
            "x15" | "a5" => Register::X15,
            "x16" | "a6" => Register::X16,
            "x17" | "a7" => Register::X17,
            //---
            "x18" | "s2" => Register::X18,
            "x19" | "s3" => Register::X19,
            "x20" | "s4" => Register::X20,
            "x21" | "s5" => Register::X21,
            "x22" | "s6" => Register::X22,
            "x23" | "s7" => Register::X23,
            "x24" | "s8" => Register::X24,
            "x25" | "s9" => Register::X25,
            "x26" | "s10" => Register::X26,
            "x27" | "s11" => Register::X27,
            //---
            "x28" | "t3" => Register::X28,
            "x29" | "t4" => Register::X29,
            "x30" | "t5" => Register::X30,
            "x31" | "t6" => Register::X31,
            _ => {
                return Err(NonExistentRegister(token.clone()));
            }
        })
    }
}
