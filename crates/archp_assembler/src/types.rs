use std::fmt::Display;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
pub enum InstructionType {
    R,
    I,
    B,
    S,
    U,
    J,
}

impl Display for InstructionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstructionType::R => write!(f, "R"),
            InstructionType::I => write!(f, "I"),
            InstructionType::B => write!(f, "B"),
            InstructionType::S => write!(f, "S"),
            InstructionType::U => write!(f, "U"),
            InstructionType::J => write!(f, "J"),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(tag = "type", content = "value")]
pub enum OperandType {
    RegD,
    RegS,
    Imm(u8, bool),
    Addr(u8),
    None,
}

impl Display for OperandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            OperandType::RegD => write!(f, "RegD"),
            OperandType::RegS => write!(f, "RegS"),
            OperandType::Imm(size, signed) => write!(f, "Imm({}, {})", size, signed),
            OperandType::Addr(size) => write!(f, "Addr({})", size),
            OperandType::None => write!(f, "None"),
        }
    }
}
