use anyhow::{Result, bail};

use crate::{
    encoder::address::{encode_address, encode_address_check},
    instructions::Entry,
    operand::OperandType,
    utils::split::split_hi_lo,
};

#[derive(Debug, Clone)]
pub enum RelocationType {
    Bits(u8),
    Low,
    High,
}

#[derive(Debug, Clone)]
pub struct Relocation<'a> {
    pub rtype: RelocationType,
    /// Offset of target instruction in the text section
    pub offset: usize,
    /// Base address for the relative address calculation.
    /// Usually the offset of the instruction itself, or to an 'auipc'
    pub base: usize,
    /// Label to be resolved
    pub label: &'a str,
    /// Addend to be added to the resolved label address
    pub addend: i64,
    /// Instruction entry for the target instruction
    pub instr: &'static Entry,
}

impl Entry {
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

                let mut ops = self.itype.decode(code);
                ops[idx] = addr;
                let word = self.itype.encode(self.opcode, self.funct3, &ops);

                return Ok(word);
            }
        }

        bail!("Instruction '{}' does not support relocation", self.name);
    }
}
