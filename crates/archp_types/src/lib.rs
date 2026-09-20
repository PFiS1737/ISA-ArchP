#[cfg(feature = "proc-macro")]
mod proc_macro;

#[derive(Debug, Clone, Copy)]
pub enum InstructionType {
    R,
    I,
    B,
    S,
    U,
    J,
}

#[derive(Debug, Clone, Copy)]
pub enum OperandType {
    RegD,
    RegS,
    Imm(u8, bool),
    Addr(u8),
    None,
}

impl InstructionType {
    pub fn default_format(&self) -> &'static [OperandType] {
        use OperandType::*;
        match self {
            InstructionType::R => &[RegD, RegS, RegS],
            InstructionType::I => &[RegD, RegS, Imm(12, true)],
            InstructionType::B => &[RegS, RegS, Addr(12)],
            InstructionType::S => &[RegS, RegS, Imm(12, true)],
            InstructionType::U => &[RegD, Imm(20, false)],
            InstructionType::J => &[RegD, Addr(20)],
        }
    }
}
