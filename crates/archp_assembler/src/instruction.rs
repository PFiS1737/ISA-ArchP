mod instructions;

use anyhow::Result;
use archp_types::{InstructionType, OperandType};
use smallvec::SmallVec;

use crate::{
    codec::{
        instruction::{decode_instruction, decode_opcode, encode_instruction},
        operands::{decode_operands, encode_operands},
    },
    context::Context,
    operand::Operand,
};

pub struct Instruction {
    pub name: &'static str,
    pub opcode: u32,
    pub funct3: u32,
    pub itype: InstructionType,
    pub format: &'static [OperandType],
}

impl Instruction {
    pub fn get_by_name(name: &str) -> Option<&'static Self> {
        instructions::get_by_name(name)
    }

    pub fn get_by_code(code: u32) -> Option<&'static Self> {
        let (opcode, funct3) = decode_opcode(code);
        instructions::get_by_opcode(opcode, funct3)
    }

    pub fn encode<'src>(
        &'static self,
        ctx: &mut Context<'src>,
        operands: &[Operand<'src>],
    ) -> Result<u32> {
        let ops = encode_operands(ctx, self.name, self.format, operands)?;
        let code = encode_instruction(&self.itype, self.opcode, self.funct3, &ops);
        Ok(code)
    }

    pub fn decode(&'static self, code: u32) -> SmallVec<[Operand<'static>; 3]> {
        let ops = decode_instruction(&self.itype, code);
        decode_operands(self.format, ops)
    }
}

#[cfg(test)]
macro instr( @($ctx:expr) $name:ident $($ops:expr),* $(;)? ) {{
    $name.encode($ctx, &$crate::operand::ops![$($ops),*])
}}

#[cfg(test)]
macro test_instr($name:ident $($ops:expr),*) {{
    use $crate::instruction::instr;
    use $crate::context::Context;
    match instr!{ @(&mut Context::test()) $name $($ops),* } {
        Ok(code) => format!("{:#010X}", code),
        Err(e) => format!("Error: {}", e),
    }
}}

#[cfg(test)]
mod tests;
