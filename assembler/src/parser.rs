use std::collections::HashMap;

use crate::{
    opcode::{
        ParsingError::{NonExistentOpcodeError, SymbolError, WrongArgumentError},
        *,
    },
    tokens::Token,
};

pub fn tokens_to_instructions(
    tokens: &[Token],
    symbol_table: &HashMap<String, u32>,
) -> Result<Vec<Mnemonic>, ParsingError> {
    let mut program: Vec<Mnemonic> = Vec::new();
    for line in tokens.split(|t| *t == Token::NewLine) {
        program.push(parse_instruction(line, symbol_table)?);
    }
    Ok(program)
}

fn parse_instruction(
    input: &[Token],
    symbol_table: &HashMap<String, u32>,
) -> std::result::Result<Mnemonic, ParsingError> {
    let mnemonic = match &input[0] {
        Token::Identifier(a) => a,
        _ => {
            return Err(SymbolError);
        }
    };

    let args = &input[1..];

    Ok(match mnemonic.as_str() {
        "add" => Mnemonic::ADD(generate_rtype(args)?),
        "sub" => Mnemonic::SUB(generate_rtype(args)?),
        "or" => Mnemonic::OR(generate_rtype(args)?),
        "and" => Mnemonic::AND(generate_rtype(args)?),
        "xor" => Mnemonic::XOR(generate_rtype(args)?),

        "addi" => Mnemonic::ADDI(generate_itype(args)?),
        "andi" => Mnemonic::ANDI(generate_itype(args)?),
        "xori" => Mnemonic::XORI(generate_itype(args)?),
        "orii" => Mnemonic::ORI(generate_itype(args)?),

        "slli" => Mnemonic::SLLI(generate_itype_shifts(args)?),
        "srli" => Mnemonic::SRLI(generate_itype_shifts(args)?),

        "lw" => Mnemonic::LW(generate_itype_memory(args)?),
        "lb" => Mnemonic::LB(generate_itype_memory(args)?),

        "sw" => Mnemonic::SW(generate_stype_memory(args)?),
        "sb" => Mnemonic::SB(generate_stype_memory(args)?),

        "beq" => Mnemonic::BEQ(generate_btype(args)?),
        "bne" => Mnemonic::BNE(generate_btype(args)?),
        "blt" => Mnemonic::BLT(generate_btype(args)?),
        "bge" => Mnemonic::BGE(generate_btype(args)?),

        "jal" => Mnemonic::JAL(generate_jtype(args)?),

        "jalr" => Mnemonic::JALR(generate_itype_jump(args)?),

        _ => return Err(NonExistentOpcodeError),
    })
}

// ---

fn generate_jtype(args: &[Token]) -> Result<JType, ParsingError> {
    todo!()
}

fn generate_btype(args: &[Token]) -> Result<BType, ParsingError> {
    todo!()
}

fn generate_stype_memory(args: &[Token]) -> Result<STypeMemory, ParsingError> {
    if args.len() != 6
        || !args[1].eq(&Token::Coma)
        || !args[3].eq(&Token::OpeningParenthesis)
        || !args[5].eq(&Token::ClosingParenthesis)
    {
        return Err(WrongArgumentError);
    }

    Ok(STypeMemory {
        source: Register::new(&args[0])?,
        offset: Offset::new(&args[2])?,
        base_address: Register::new(&args[4])?,
    })
}

fn generate_itype_memory(args: &[Token]) -> Result<ITypeMemory, ParsingError> {
    if args.len() != 6
        || !args[1].eq(&Token::Coma)
        || !args[3].eq(&Token::OpeningParenthesis)
        || !args[5].eq(&Token::ClosingParenthesis)
    {
        return Err(WrongArgumentError);
    }

    Ok(ITypeMemory {
        destination: Register::new(&args[0])?,
        offset: Offset::new(&args[2])?,
        base_address: Register::new(&args[4])?,
    })
}

fn generate_itype_shifts(args: &[Token]) -> Result<ITypeShifts, ParsingError> {
    if args.len() != 5 || !args[1].eq(&Token::Coma) || !args[3].eq(&Token::Coma) {
        return Err(WrongArgumentError);
    }

    Ok(ITypeShifts {
        destination: Register::new(&args[0])?,
        source: Register::new(&args[2])?,
        shamt: Shamt::new(&args[4])?,
    })
}

fn generate_itype_jump(args: &[Token]) -> Result<ITypeJump, ParsingError> {
    todo!()
}

fn generate_itype(args: &[Token]) -> Result<IType, ParsingError> {
    if args.len() != 5 || !args[1].eq(&Token::Coma) || !args[3].eq(&Token::Coma) {
        return Err(WrongArgumentError);
    }

    Ok(IType {
        destination: Register::new(&args[0])?,
        source: Register::new(&args[2])?,
        immediate: Immediate::new(&args[4])?,
    })
}

fn generate_rtype(args: &[Token]) -> Result<RType, ParsingError> {
    if args.len() != 5 || !args[1].eq(&Token::Coma) || !args[3].eq(&Token::Coma) {
        return Err(WrongArgumentError);
    }

    Ok(RType {
        destination: Register::new(&args[0])?,
        first_source: Register::new(&args[2])?,
        second_source: Register::new(&args[4])?,
    })
}
