use std::collections::HashMap;

use crate::structures::{ParsingError::*, *};

pub fn parse(
    tokens: &[Token],
    symbol_table: &HashMap<String, usize>,
) -> Result<Vec<Instruction>, ParsingError> {
    let mut statements: Vec<Instruction> = Vec::new();
    for (index, line) in tokens.split(|t| *t == Token::NewLine).enumerate() {
        if line.is_empty() {
            continue;
        }
        let pc_counter = index * 4;
        statements.push(parse_statements(line, symbol_table, pc_counter)?);
    }
    Ok(statements)
}

fn parse_statements(
    tokens: &[Token],
    symbol_table: &HashMap<String, usize>,
    pc_counter: usize,
) -> std::result::Result<Instruction, ParsingError> {
    let mnemonic = match &tokens[0] {
        Token::Identifier(a) => a,
        _ => {
            return Err(SymbolError);
        }
    };

    let operands = &tokens[1..];

    Ok(match mnemonic.as_str() {
        "add" => Instruction::ADD(generate_rtype(operands)?),
        "sub" => Instruction::SUB(generate_rtype(operands)?),
        "or" => Instruction::OR(generate_rtype(operands)?),
        "and" => Instruction::AND(generate_rtype(operands)?),
        "xor" => Instruction::XOR(generate_rtype(operands)?),

        "addi" => Instruction::ADDI(generate_itype(operands)?),
        "andi" => Instruction::ANDI(generate_itype(operands)?),
        "xori" => Instruction::XORI(generate_itype(operands)?),
        "orii" => Instruction::ORI(generate_itype(operands)?),

        "slli" => Instruction::SLLI(generate_itype_shifts(operands)?),
        "srli" => Instruction::SRLI(generate_itype_shifts(operands)?),

        "lw" => Instruction::LW(generate_itype_memory(operands)?),
        "lb" => Instruction::LB(generate_itype_memory(operands)?),

        "sw" => Instruction::SW(generate_stype_memory(operands)?),
        "sb" => Instruction::SB(generate_stype_memory(operands)?),

        "beq" => Instruction::BEQ(generate_btype(operands, pc_counter, symbol_table)?),
        "bne" => Instruction::BNE(generate_btype(operands, pc_counter, symbol_table)?),
        "blt" => Instruction::BLT(generate_btype(operands, pc_counter, symbol_table)?),
        "bge" => Instruction::BGE(generate_btype(operands, pc_counter, symbol_table)?),

        "jal" => Instruction::JAL(generate_jtype(operands, pc_counter, symbol_table)?),

        "jalr" => Instruction::JALR(generate_itype_jump(operands)?),

        _ => return Err(NonExistentOpcodeError),
    })
}

// ---

fn generate_jtype(
    operands: &[Token],
    pc_counter: usize,
    symbol_table: &HashMap<String, usize>,
) -> Result<JType, ParsingError> {
    if operands.len() != 3 || !operands[1].eq(&Token::Coma) {
        return Err(WrongArgumentError);
    }

    Ok(JType {
        destination: Register::new(&operands[0])?,
        big_label: BigLabel::new(&operands[2], symbol_table, pc_counter)?,
    })
}

fn generate_btype(
    operands: &[Token],
    pc_counter: usize,
    symbol_table: &HashMap<String, usize>,
) -> Result<BType, ParsingError> {
    if operands.len() != 5 || !operands[1].eq(&Token::Coma) || !operands[3].eq(&Token::Coma) {
        return Err(WrongArgumentError);
    }

    Ok(BType {
        first_source: Register::new(&operands[0])?,
        second_source: Register::new(&operands[2])?,
        label: Label::new(&operands[4], symbol_table, pc_counter)?,
    })
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
