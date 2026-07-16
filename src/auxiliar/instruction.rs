use crate::auxiliar::operands::{BigLabel, Immediate, Label, Offset, Register, Shamt};

#[derive(PartialEq, Eq, Debug)]
#[allow(clippy::upper_case_acronyms)]
pub(crate) enum Instruction {
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
pub(crate) struct RType {
    pub destination: Register,
    pub first_source: Register,
    pub second_source: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct IType {
    pub destination: Register,
    pub source: Register,
    pub immediate: Immediate,
}
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct ITypeShifts {
    pub destination: Register,
    pub source: Register,
    pub shamt: Shamt,
}
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct ITypeMemory {
    pub destination: Register,
    pub offset: Offset,
    pub base_address: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct STypeMemory {
    pub source: Register,
    pub offset: Offset,
    pub base_address: Register,
}
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct BType {
    pub first_source: Register,
    pub second_source: Register,
    pub label: Label,
}
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct JType {
    pub destination: Register,
    pub big_label: BigLabel,
}
#[derive(PartialEq, Eq, Debug)]
pub(crate) struct ITypeJump {
    pub destination: Register,
    pub offset: Offset,
    pub target_address: Register,
}
