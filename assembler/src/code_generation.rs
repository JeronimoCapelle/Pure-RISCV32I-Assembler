use crate::structures::{
    IType, ITypeShifts, Immediate, Instruction, RType, Register, STypeMemory, Shamt,
};

pub fn emit(instructions: Vec<Instruction>) -> Vec<u32> {
    let mut buffer: Vec<u32> = Vec::new();

    for i in instructions {
        buffer.push(encode_instruction(i));
    }

    buffer
}

fn encode_instruction(instruction: Instruction) -> u32 {
    match instruction {
        Instruction::ADDI(itype) => {
            let funct3 = 0;
            generate_itype(funct3, itype)
        }

        Instruction::ADD(rtype) => {
            let funct3 = 0;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }
        Instruction::SUB(rtype) => {
            let funct3 = 0;
            let funct7 = 32;
            generate_rtype(funct3, funct7, rtype)
        }
        Instruction::BNE(btype) => {
            let opcode = 99;
            let funct3 = 1;
        }
        Instruction::BEQ(btype) => {
            let opcode = 99;
            let funct3 = 0;
        }
        Instruction::BLT(btype) => {
            let opcode = 99;
            let funct3 = 4;
        }
        Instruction::BGE(btype) => {
            let opcode = 99;
            let funct3 = 5;
        }
        Instruction::JAL(jtype) => {
            let opcode = 111;
        }
        Instruction::JALR(itype_jump) => {
            let opcode = 103;
            let funct1 = 0;
        }
        Instruction::LW(itype_memory) => {
            let opcode = 3;
            let funct3 = 2;
        }
        Instruction::SW(stype_memory) => {
            let funct3 = 2;
            generate_stype_memory(funct3, stype_memory)
        }
        Instruction::LB(itype_memory) => {
            let opcode = 3;
            let funct3 = 0;
        }
        Instruction::SB(stype_memory) => {
            let funct3 = 0;
            generate_stype_memory(funct3, stype_memory)
        }
        Instruction::SLLI(itype_shifts) => {
            let funct3 = 1;
            let funct7 = 0;
            generate_itype_shifts(funct3, funct7, itype_shifts)
        }
        Instruction::SRLI(itype_shifts) => {
            let funct3 = 5;
            let funct7 = 0;
            generate_itype_shifts(funct3, funct7, itype_shifts)
        }
        Instruction::AND(rtype) => {
            let funct3 = 7;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }
        Instruction::OR(rtype) => {
            let funct3 = 6;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }
        Instruction::XOR(rtype) => {
            let funct3 = 4;
            let funct7 = 0;
            generate_rtype(funct3, funct7, rtype)
        }
        Instruction::ANDI(itype) => {
            let funct3 = 7;
            generate_itype(funct3, itype)
        }
        Instruction::ORI(itype) => {
            let funct3 = 6;
            generate_itype(funct3, itype)
        }
        Instruction::XORI(itype) => {
            let funct3 = 4;
            generate_itype(funct3, itype)
        }
    }
}

fn encode_register(register: Register) -> u32 {
    match register {
        Register::X0 => 0,
        Register::X1 => 1,
        Register::X2 => 2,
        Register::X3 => 3,
        Register::X4 => 4,
        Register::X5 => 5,
        Register::X6 => 6,
        Register::X7 => 7,
        Register::X8 => 8,
        Register::X9 => 9,
        Register::X10 => 10,
        Register::X11 => 11,
        Register::X12 => 12,
        Register::X13 => 13,
        Register::X14 => 14,
        Register::X15 => 15,
        Register::X16 => 16,
        Register::X17 => 17,
        Register::X18 => 18,
        Register::X19 => 19,
        Register::X20 => 20,
        Register::X21 => 21,
        Register::X22 => 22,
        Register::X23 => 23,
        Register::X24 => 24,
        Register::X25 => 25,
        Register::X26 => 26,
        Register::X27 => 27,
        Register::X28 => 28,
        Register::X29 => 29,
        Register::X30 => 30,
        Register::X31 => 31,
    }
}

fn generate_rtype(funct3: u32, funct7: u32, rtype: RType) -> u32 {
    let opcode = 51;
    let destination = encode_register(rtype.destination) << 7;
    let funct3 = funct3 << 12;
    let first_source = encode_register(rtype.first_source) << 15;
    let second_source = encode_register(rtype.second_source) << 20;
    let funct7 = funct7 << 25;

    funct7 | second_source | first_source | funct3 | destination | opcode
}

fn generate_itype(funct3: u32, itype: IType) -> u32 {
    let opcode = 19;
    let destination = encode_register(itype.destination) << 7;
    let funct3 = funct3 << 12;
    let source = encode_register(itype.source) << 15;
    let immediate = itype.immediate.encode() << 20;

    immediate | source | funct3 | destination | opcode
}

fn generate_itype_shifts(funct3: u32, funct7: u32, itype_shifts: ITypeShifts) -> u32 {
    let opcode = 19;
    let destination = encode_register(itype_shifts.destination) << 7;
    let funct3 = funct3 << 12;
    let source = encode_register(itype_shifts.source) << 15;
    let shamt = itype_shifts.shamt.encode() << 20;
    let funct7 = funct7 << 25;

    funct7 | shamt | source | funct3 | destination | opcode
}

fn generate_stype_memory(funct3: u32, stype_memory: STypeMemory) -> u32 {
    let opcode = 35;
    let offset_1 = (stype_memory.offset.encode() & 0b11111) << 7;
    let funct3 = funct3 << 12;
    let source = encode_register(stype_memory.source) << 15;
    let base_address = encode_register(stype_memory.base_address) << 20;
    let offset_2 = (stype_memory.offset.encode() & 0b_111111100000) << 25;

    offset_2 | base_address | source | funct3 | offset_1 | opcode
}
