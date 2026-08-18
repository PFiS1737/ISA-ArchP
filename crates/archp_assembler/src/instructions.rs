mod arithmetic_logic;
mod branch;
mod jump_and_link;
mod load_store;
mod mul_div;
mod set;
mod shift_rotate;
mod stack_call_return;
mod system;
mod types;
mod upper_imm;

use std::{collections::HashMap, sync::LazyLock};

use anyhow::{Result, bail};
use smallvec::SmallVec;

use crate::{
    context::Context,
    encoder::{
        address::encode_address_as, immediate::encode_immediate_as, register::encode_register,
    },
    instructions::types::InstrType,
    operand::{Operand, OperandType},
};

inventory::collect!(Entry);

pub static INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static Entry>> =
    LazyLock::new(|| HashMap::from_iter(inventory::iter::<Entry>.into_iter().map(|e| (e.name, e))));

pub struct Entry {
    name: &'static str,
    opcode: u32,
    funct3: u32,
    itype: InstrType,
    format: &'static [OperandType],
}

trait Instruction: Send + Sync {
    const NAME: &'static str;
    const OPCODE: u32;
    const FUNCT3: u32;
    const ITYPE: InstrType;
    const FORMAT: &'static [OperandType];
}

impl Entry {
    const fn of<T: Instruction>() -> Self {
        Self {
            name: T::NAME,
            opcode: T::OPCODE,
            funct3: T::FUNCT3,
            itype: T::ITYPE,
            format: T::FORMAT,
        }
    }

    pub fn encode<'src>(
        &'static self,
        ctx: &mut Context<'src>,
        operands: &[Operand<'src>],
    ) -> Result<u32> {
        let operands = self.parse(ctx, operands)?;

        Ok(self.itype.encode(self.opcode, self.funct3, &operands))
    }

    pub fn apply_relocation(&self, code: &mut u32, addr: i64, base: u32) -> Result<()> {
        for (idx, op_ty) in self.format.iter().enumerate() {
            if let OperandType::Imm(_, signed) = op_ty
                && !*signed
            {
                bail!(
                    "Instruction '{}' does not support relocation for unsigned immediate",
                    self.name
                );
            }

            if let OperandType::Addr(bits) | OperandType::Imm(bits, _) = op_ty {
                let addr =
                    encode_address_as(addr, *bits, base, matches!(op_ty, OperandType::Addr(..)))?;

                let mut ops = self.itype.decode(*code);
                ops[idx] = addr;
                *code = self.itype.encode(self.opcode, self.funct3, &ops);
                return Ok(());
            }
        }

        bail!("Instruction '{}' does not support relocation", self.name);
    }

    fn parse<'src>(
        &'static self,
        ctx: &mut Context<'src>,
        operands: &[Operand<'src>],
    ) -> Result<SmallVec<[u32; 3]>> {
        let expected = self
            .format
            .iter()
            .filter(|x| !matches!(x, OperandType::None))
            .count();
        self.assert_operand_count(operands.len(), expected)?;

        let mut ops = operands.iter();

        let mut ret = SmallVec::new();

        for op_ty in self.format {
            let val = match *op_ty {
                OperandType::RegD | OperandType::RegS => encode_register(ctx, ops.next().unwrap())?,
                OperandType::Imm(bits, signed) => {
                    encode_immediate_as(ops.next().unwrap(), bits, signed)?
                },
                OperandType::Addr(..) => {
                    let pc = ctx.codes.len() * 4;
                    // TODO: can we put the addend into the instruction code?
                    ctx.add_relocation(self, pc, ops.next().unwrap())?;
                    0
                },
                OperandType::None => 0,
            };

            ret.push(val);
        }

        Ok(ret)
    }

    fn assert_operand_count(&self, count: usize, expected: usize) -> Result<()> {
        if count != expected {
            bail!(
                "Instruction '{}' requires {} operands, got {}",
                self.name,
                expected,
                count
            );
        }

        Ok(())
    }
}

macro instruction {
    (@impl
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            funct3: $funct3:literal,
            itype: $itype:ident,
            format: $format:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::instructions::Instruction for $id {
            const NAME: &'static str = $name;
            const OPCODE: u32 = $opcode;
            const FUNCT3: u32 = $funct3;
            const ITYPE: $crate::instructions::InstrType = $crate::instructions::InstrType::$itype;
            const FORMAT: &'static [$crate::operand::OperandType] = $format;
        }

        inventory::submit! {
            $crate::instructions::Entry::of::<$id>()
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            funct3: $funct3:literal,
            itype: $itype:ident,
        }
    ) => {
        instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: $funct3,
                itype: $itype,
                format: instruction!(@fmt $itype),
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            itype: $itype:ident,
        }
    ) => {
        instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: 0,
                itype: $itype,
                format: instruction!(@fmt $itype),
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            funct3: $funct3:literal,
            itype: $itype:ident,
            format: $format:tt,
        }
    ) => {
        instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: $funct3,
                itype: $itype,
                format: $crate::operand::op_types! $format,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            itype: $itype:ident,
            format: $format:tt,
        }
    ) => {
        instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: 0,
                itype: $itype,
                format: $crate::operand::op_types! $format,
            }
        }
    },

    (@fmt R) => { $crate::operand::op_types![RegD, RegS, RegS] },
    (@fmt I) => { $crate::operand::op_types![RegD, RegS, Imm(12, i)] },
    (@fmt B) => { $crate::operand::op_types![RegS, RegS, Addr(12)] },
    (@fmt S) => { $crate::operand::op_types![RegS, RegS, Imm(12, i)] },
    (@fmt U) => { $crate::operand::op_types![RegD, Imm(20, u)] },
    (@fmt J) => { $crate::operand::op_types![RegD, Addr(20)] },
}

#[cfg(test)]
macro instr( @($ctx:expr) $name:ident $($ops:expr),* $(;)? ) {{
    let name = <$name as $crate::instructions::Instruction>::NAME;
    let instr = $crate::instructions::INSTRUCTIONS.get(name).unwrap();
    instr.encode($ctx, &$crate::operand::ops![$($ops),*])
}}

#[cfg(test)]
macro test_instr($name:ident $($ops:expr),*) {{
    use $crate::instructions::instr;
    use $crate::context::Context;
    match instr!{ @(&mut Context::test()) $name $($ops),* } {
        Ok(code) => format!("{:#010X}", code),
        Err(e) => format!("Error: {}", e),
    }
}}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    #[test]
    fn encode_r() {
        use arithmetic_logic::Add;

        assert_snapshot!(test_instr!(Add "r1", "r2"), @"Error: Instruction 'add' requires 3 operands, got 2");
        assert_snapshot!(test_instr!(Add "r1", "r2", "r3", "r4"), @"Error: Instruction 'add' requires 3 operands, got 4");
        assert_snapshot!(test_instr!(Add "r1", "r2", "rrr"), @"Error: Invalid register: rrr");
        assert_snapshot!(test_instr!(Add "r1", "r2", 123), @"Error: Invalid register: 123");

        assert_snapshot!(test_instr!(Add "r1", "r2", "r3"), @"0x00022003");
    }

    #[test]
    fn encode_i() {
        use arithmetic_logic::Addi;

        assert_snapshot!(test_instr!(Addi "r1", "r2"), @"Error: Instruction 'addi' requires 3 operands, got 2");
        assert_snapshot!(test_instr!(Addi "r1", "r2", "r3", "r4"), @"Error: Instruction 'addi' requires 3 operands, got 4");
        assert_snapshot!(test_instr!(Addi "r1", "rrr", 123), @"Error: Invalid register: rrr");
        assert_snapshot!(test_instr!(Addi "r1", "r2", "r3"), @"Error: Expected immediate, got: r3");
        assert_snapshot!(test_instr!(Addi "r1", "r2", 0xFFF), @"Error: Immediate '4095' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(test_instr!(Addi "r1", "r2", 0x7FF), @"0x010227FF");
        assert_snapshot!(test_instr!(Addi "r1", "r2", 0xFFFF), @"Error: Immediate '65535' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(test_instr!(Addi "r1", "r2", 0xFFFFFFFF_i64), @"0x01022FFF");
        assert_snapshot!(test_instr!(Addi "r1", "r2", -1), @"0x01022FFF");

        assert_snapshot!(test_instr!(Addi "r1", "r2", 3), @"0x01022003");
        assert_snapshot!(test_instr!(Addi "r1", "r2", 2047), @"0x010227FF");
        assert_snapshot!(test_instr!(Addi "r1", "r2", 2048), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(test_instr!(Addi "r1", "r2", -3), @"0x01022FFD");
        assert_snapshot!(test_instr!(Addi "r1", "r2", -2048), @"0x01022800");
        assert_snapshot!(test_instr!(Addi "r1", "r2", -2049), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");

        use shift_rotate::Srli;

        assert_snapshot!(test_instr!(Srli "r1", "r2", 32), @"Error: Immediate '32' out of range for u5 (0 ..= 31)");
        assert_snapshot!(test_instr!(Srli "r1", "r2", 31), @"0x0A42201F");
    }

    #[test]
    fn enocde_b() {
        use load_store::Sw;

        assert_snapshot!(test_instr!(Sw "r1", "r2", 3), @"0x11402061");
        assert_snapshot!(test_instr!(Sw "r1", "r2", 2047), @"0x115E2FE1");
        assert_snapshot!(test_instr!(Sw "r1", "r2", 2048), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(test_instr!(Sw "r1", "r2", -3), @"0x117E2FA1");
        assert_snapshot!(test_instr!(Sw "r1", "r2", -2048), @"0x11602001");
        assert_snapshot!(test_instr!(Sw "r1", "r2", -2049), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");
    }

    #[test]
    fn encode_u() {
        use upper_imm::Lui;

        assert_snapshot!(test_instr!(Lui "r1"), @"Error: Instruction 'lui' requires 2 operands, got 1");
        assert_snapshot!(test_instr!(Lui "r1", "r2", "r3"), @"Error: Instruction 'lui' requires 2 operands, got 3");
        assert_snapshot!(test_instr!(Lui "r1", "r2"), @"Error: Expected immediate, got: r2");
        assert_snapshot!(test_instr!(Lui "r3", 0x200000), @"Error: Immediate '2097152' out of range for u20 (0 ..= 1048575)");
        assert_snapshot!(test_instr!(Lui "r3", -123), @"Error: Immediate '-123' out of range for u20 (0 ..= 1048575)");

        assert_snapshot!(test_instr!(Lui "r3", 0xABCDE), @"0x1746BCDE");
    }
}
