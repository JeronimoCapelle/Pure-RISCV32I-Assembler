use std::collections::HashMap;

use crate::{
    structures::Token,
    structures::{
        ParsingError::{NonExistentOpcodeError, SymbolError, WrongArgumentError},
        *,
    },
};

pub fn parse(
    tokens: &[Token],
    symbol_table: &HashMap<String, u32>,
) -> Result<Vec<Mnemonic>, ParsingError> {
    let mut statements: Vec<Mnemonic> = Vec::new();
    for line in tokens.split(|t| *t == Token::NewLine) {
        statements.push(parse_statements(line, symbol_table)?);
    }
    Ok(statements)
}

fn parse_statements(
    tokens: &[Token],
    symbol_table: &HashMap<String, u32>,
) -> std::result::Result<Mnemonic, ParsingError> {
    let mnemonic = match &tokens[0] {
        Token::Identifier(a) => a,
        _ => {
            return Err(SymbolError);
        }
    };

    let operands = &tokens[1..];

    Ok(match mnemonic.as_str() {
        "add" => Mnemonic::ADD(generate_rtype(operands)?),
        "sub" => Mnemonic::SUB(generate_rtype(operands)?),
        "or" => Mnemonic::OR(generate_rtype(operands)?),
        "and" => Mnemonic::AND(generate_rtype(operands)?),
        "xor" => Mnemonic::XOR(generate_rtype(operands)?),

        "addi" => Mnemonic::ADDI(generate_itype(operands)?),
        "andi" => Mnemonic::ANDI(generate_itype(operands)?),
        "xori" => Mnemonic::XORI(generate_itype(operands)?),
        "orii" => Mnemonic::ORI(generate_itype(operands)?),

        "slli" => Mnemonic::SLLI(generate_itype_shifts(operands)?),
        "srli" => Mnemonic::SRLI(generate_itype_shifts(operands)?),

        "lw" => Mnemonic::LW(generate_itype_memory(operands)?),
        "lb" => Mnemonic::LB(generate_itype_memory(operands)?),

        "sw" => Mnemonic::SW(generate_stype_memory(operands)?),
        "sb" => Mnemonic::SB(generate_stype_memory(operands)?),

        "beq" => Mnemonic::BEQ(generate_btype(operands)?),
        "bne" => Mnemonic::BNE(generate_btype(operands)?),
        "blt" => Mnemonic::BLT(generate_btype(operands)?),
        "bge" => Mnemonic::BGE(generate_btype(operands)?),

        "jal" => Mnemonic::JAL(generate_jtype(operands)?),

        "jalr" => Mnemonic::JALR(generate_itype_jump(operands)?),

        _ => return Err(NonExistentOpcodeError),
    })
}

// ---

fn generate_jtype(operands: &[Token]) -> Result<JType, ParsingError> {
    todo!()
}

fn generate_btype(operands: &[Token]) -> Result<BType, ParsingError> {
    todo!()
}

fn generate_stype_memory(operands: &[Token]) -> Result<STypeMemory, ParsingError> {
    if operands.len() != 6
        || !operands[1].eq(&Token::Coma)
        || !operands[3].eq(&Token::OpeningParenthesis)
        || !operands[5].eq(&Token::ClosingParenthesis)
    {
        return Err(WrongArgumentError);
    }

    Ok(STypeMemory {
        source: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        base_address: Register::new(&operands[4])?,
    })
}

fn generate_itype_memory(operands: &[Token]) -> Result<ITypeMemory, ParsingError> {
    if operands.len() != 6
        || !operands[1].eq(&Token::Coma)
        || !operands[3].eq(&Token::OpeningParenthesis)
        || !operands[5].eq(&Token::ClosingParenthesis)
    {
        return Err(WrongArgumentError);
    }

    Ok(ITypeMemory {
        destination: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        base_address: Register::new(&operands[4])?,
    })
}

fn generate_itype_shifts(operands: &[Token]) -> Result<ITypeShifts, ParsingError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Coma) || !operands[3].eq(&Token::Coma) {
        return Err(WrongArgumentError);
    }

    Ok(ITypeShifts {
        destination: Register::new(&operands[0])?,
        source: Register::new(&operands[2])?,
        shamt: Shamt::new(&operands[4])?,
    })
}

fn generate_itype_jump(operands: &[Token]) -> Result<ITypeJump, ParsingError> {
    if operands.len() != 6
        || !operands[1].eq(&Token::Coma)
        || !operands[3].eq(&Token::OpeningParenthesis)
        || !operands[5].eq(&Token::ClosingParenthesis)
    {
        return Err(WrongArgumentError);
    }

    Ok(ITypeJump {
        destination: Register::new(&operands[0])?,
        offset: Offset::new(&operands[2])?,
        target_address: Register::new(&operands[4])?,
    })
}

fn generate_itype(operands: &[Token]) -> Result<IType, ParsingError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Coma) || !operands[3].eq(&Token::Coma) {
        return Err(WrongArgumentError);
    }

    Ok(IType {
        destination: Register::new(&operands[0])?,
        source: Register::new(&operands[2])?,
        immediate: Immediate::new(&operands[4])?,
    })
}

fn generate_rtype(operands: &[Token]) -> Result<RType, ParsingError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Coma) || !operands[3].eq(&Token::Coma) {
        return Err(WrongArgumentError);
    }

    Ok(RType {
        destination: Register::new(&operands[0])?,
        first_source: Register::new(&operands[2])?,
        second_source: Register::new(&operands[4])?,
    })
}
