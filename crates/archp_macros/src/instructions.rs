mod codegen;
mod parser;

use archp_types::{InstructionType, OperandType};
use syn::{Attribute, Ident, LitInt, Token, punctuated::Punctuated};

#[derive(Debug, Clone)]
pub struct Root {
    opcodes: Punctuated<Opcode, Token![;]>,
}

#[derive(Debug, Clone)]
struct Opcode {
    attrs: Vec<Attribute>,
    opcode: LitInt,
    instructions: Punctuated<Instruction, Token![;]>,
}

#[derive(Debug, Clone)]
struct Instruction {
    attrs: Vec<Attribute>,
    funct3: Option<LitInt>,
    itype: InstructionType,
    name: Ident,
    formats: Option<Punctuated<OperandType, Token![,]>>,
}
