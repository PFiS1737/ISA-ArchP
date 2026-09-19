use anyhow::{Result, bail};

use crate::{
    codec::{
        address::{encode_address, encode_address_check},
        instruction::{decode_instruction, encode_instruction},
    },
    context::Context,
    instructions::Instruction,
    operand::Operand,
    types::OperandType,
    utils::split::split_hi_lo,
};

#[derive(Debug, Clone)]
pub enum RelocationType {
    Bits(u8),
    Low,
    High,
}

#[derive(Debug, Clone)]
pub struct Relocation<'src> {
    pub rtype: RelocationType,
    /// Offset of target instruction in the text section
    pub offset: usize,
    /// Base address for the relative address calculation.
    /// Usually the offset of the instruction itself, or to an 'auipc'
    pub base: usize,
    /// Label to be resolved
    pub label: &'src str,
    /// Addend to be added to the resolved label address
    pub addend: i64,
    /// Instruction entry for the target instruction
    pub instr: &'src str,
}

impl<'src> Context<'src> {
    pub fn add_relocation(
        &mut self,
        instr: &'src str,
        rtype: RelocationType,
        offset: usize,
        base: usize,
        op: &Operand<'src>,
    ) -> Result<()> {
        let (label, addend) = op.cast_address()?;

        self.relocations.push(Relocation {
            rtype,
            offset,
            base,
            label,
            addend,
            instr,
        });

        Ok(())
    }

    pub fn add_auipc_relocation(
        &mut self,
        low_instr: &'static str,
        op: &Operand<'src>,
    ) -> Result<()> {
        let offset = self.text.len();

        self.add_relocation("auipc", RelocationType::High, offset, offset, op)?;
        self.add_relocation(low_instr, RelocationType::Low, offset + 4, offset, op)?;

        Ok(())
    }
}

impl Instruction {
    pub fn apply_relocation(
        &self,
        rtype: RelocationType,
        code: u32,
        addr: i64,
        base: u32,
    ) -> Result<u32> {
        for (idx, op_ty) in self.format.iter().enumerate() {
            if let OperandType::Addr(..) | OperandType::Imm(..) = op_ty {
                let shift = matches!(op_ty, OperandType::Addr(..));

                let v = encode_address(addr, base, shift);
                let (lo, hi) = split_hi_lo(v, 12, true);

                let addr = match rtype {
                    RelocationType::Bits(bits) => encode_address_check(addr, base, shift, bits)?,
                    RelocationType::Low => lo,
                    RelocationType::High => hi,
                };

                let mut ops = decode_instruction(self.itype, code);
                ops[idx] = addr;
                let word = encode_instruction(self.itype, self.opcode, self.funct3, &ops);

                return Ok(word);
            }
        }

        bail!("Instruction '{}' does not support relocation", self.name);
    }
}
