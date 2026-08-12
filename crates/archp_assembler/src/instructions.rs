mod arithmetic_logic;
mod branch;
mod jump_and_link;
mod load_store;
mod mul_div;
mod set;
mod shift_rotate;
mod stack_call_return;
mod system;
mod upper_imm;

use std::{collections::HashMap, fmt::Display, sync::LazyLock};

use anyhow::{Result, bail};

use crate::{
    assembler::Context,
    encoder::{address::encode_address, immediate::encode_immediate_as, register::encode_register},
    operand::{Operand, OperandType},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum InstrType {
    R,
    I,
    B,
    S,
    U,
    J,
}

impl Display for InstrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstrType::R => write!(f, "R"),
            InstrType::I => write!(f, "I"),
            InstrType::B => write!(f, "B"),
            InstrType::S => write!(f, "S"),
            InstrType::U => write!(f, "U"),
            InstrType::J => write!(f, "J"),
        }
    }
}

inventory::collect!(Entry);

pub static INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static Entry>> =
    LazyLock::new(|| HashMap::from_iter(inventory::iter::<Entry>.into_iter().map(|e| (e.name, e))));

pub struct Entry {
    name: &'static str,
    opcode: u32,
    funct3: u32,
    itype: InstrType,
    operands_format: &'static [Option<OperandType>],
}

trait Instruction: Send + Sync {
    const NAME: &'static str;
    const OPCODE: u32;
    const FUNCT3: u32;
    const ITYPE: InstrType;
    const OPERANDS_FORMAT: &'static [Option<OperandType>];
}

impl Entry {
    const fn of<T: Instruction>() -> Self {
        Self {
            name: T::NAME,
            opcode: T::OPCODE,
            funct3: T::FUNCT3,
            itype: T::ITYPE,
            operands_format: T::OPERANDS_FORMAT,
        }
    }

    pub fn encode(&self, ctx: &Context, pc: u32, operands: &[Operand]) -> Result<u32> {
        let operands = self.parse(ctx, pc, operands)?;

        match self.itype {
            InstrType::R => self.encode_r(&operands),
            InstrType::I => self.encode_i(&operands),
            InstrType::B => self.encode_b(&operands),
            InstrType::S => self.encode_s(&operands),
            InstrType::U => self.encode_u(&operands),
            InstrType::J => self.encode_j(&operands),
        }
    }

    fn parse(&self, ctx: &Context, pc: u32, operands: &[Operand]) -> Result<Vec<u32>> {
        let format = self.operands_format;

        let expected = format.iter().filter(|x| x.is_some()).count();
        self.assert_operand_count(operands.len(), expected)?;

        let mut ret = Vec::with_capacity(format.len());
        let mut op_idx = 0;

        for slot in format {
            match slot {
                Some(op_ty) => {
                    let op = &operands[op_idx];
                    op_idx += 1;

                    let val = match *op_ty {
                        OperandType::RegD | OperandType::RegS => encode_register(ctx, op)?,
                        OperandType::Imm(bits, signed) => {
                            encode_immediate_as(ctx, op, bits, signed)?
                        },
                        OperandType::Addr(bits) => encode_address(ctx, op)?.as_field(bits, pc)?,
                    };

                    ret.push(val);
                },
                None => {
                    ret.push(0);
                },
            }
        }

        Ok(ret)
    }

    fn encode_r(&self, ops: &[u32]) -> Result<u32> {
        let rd = ops[0];
        let rs1 = ops[1];
        let rs2 = ops[2];

        field! {
            self.opcode, 25, 7;
            self.funct3, 22, 3;
            rd, 17, 5;
            rs1, 12, 5;
            0, 5, 7;
            rs2, 0, 5;
        }
    }

    fn encode_i(&self, ops: &[u32]) -> Result<u32> {
        let rd = ops[0];
        let rs1 = ops[1];
        let imm12 = ops[2];

        field! {
            self.opcode, 25, 7;
            self.funct3, 22, 3;
            rd, 17, 5;
            rs1, 12, 5;
            imm12, 0, 12;
        }
    }

    fn encode_b(&self, ops: &[u32]) -> Result<u32> {
        let rs1 = ops[0];
        let rs2 = ops[1];
        let offset12 = ops[2];

        field! {
            self.opcode, 25, 7;
            self.funct3, 22, 3;
            (offset12 >> 7), 17, 5;
            rs1, 12, 5;
            (offset12 & 0x7F), 5, 7;
            rs2, 0, 5;
        }
    }

    fn encode_s(&self, ops: &[u32]) -> Result<u32> {
        self.encode_b(&[ops[1], ops[0], ops[2]])
    }

    fn encode_u(&self, ops: &[u32]) -> Result<u32> {
        let rd = ops[0];
        let imm20 = ops[1];

        field! {
            self.opcode, 25, 7;
            (imm20 >> 17), 22, 3;
            rd, 17, 5;
            (imm20 & 0x1FFFF), 0, 17;
        }
    }

    fn encode_j(&self, ops: &[u32]) -> Result<u32> {
        self.encode_u(ops)
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

macro field {
    ( $( $value:expr, $shift:literal, $len:literal );* $(;)? ) => {
        Ok(
            $(
                field!(@one, $value, $shift, $len)
            )|*
        )
    },

    (@one, $value:expr, $shift:literal, $len:literal) => {{
        ((($value) as u32) & ((1u32 << $len) - 1)) << $shift
    }},
}

macro instruction {
    (@impl
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            funct3: $funct3:literal,
            itype: $itype:ident,
            operands_format: $format:expr,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::instructions::Instruction for $id {
            const NAME: &'static str = $name;
            const OPCODE: u32 = $opcode;
            const FUNCT3: u32 = $funct3;
            const ITYPE: $crate::instructions::InstrType = $crate::instructions::InstrType::$itype;
            const OPERANDS_FORMAT: &'static [Option<$crate::operand::OperandType>] = $format;
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
                operands_format: instruction!(@fmt $itype),
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
                operands_format: instruction!(@fmt $itype),
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
            operands_format: $format:tt,
        }
    ) => {
        instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: $funct3,
                itype: $itype,
                operands_format: $crate::operand::op_fmt! $format,
            }
        }
    },

    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            itype: $itype:ident,
            operands_format: $format:tt,
        }
    ) => {
        instruction! {@impl
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: 0,
                itype: $itype,
                operands_format: $crate::operand::op_fmt! $format,
            }
        }
    },

    (@fmt R) => { $crate::operand::op_fmt![RegD, RegS, RegS] },
    (@fmt I) => { $crate::operand::op_fmt![RegD, RegS, Imm(12, i)] },
    (@fmt B) => { $crate::operand::op_fmt![RegS, RegS, Addr(12)] },
    (@fmt S) => { $crate::operand::op_fmt![RegS, RegS, Imm(12, i)] },
    (@fmt U) => { $crate::operand::op_fmt![RegD, Imm(20, u)] },
    (@fmt J) => { $crate::operand::op_fmt![RegD, Addr(20)] },
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use crate::testkit::*;

    #[test]
    fn encode_r() {
        let cmd = instr("add");

        assert_snapshot!(cmd(&["r1", "r2"]), @"Error: Instruction 'add' requires 3 operands, got 2");
        assert_snapshot!(cmd(&["r1", "r2", "r3", "r4"]), @"Error: Instruction 'add' requires 3 operands, got 4");
        assert_snapshot!(cmd(&["r1", "r2", "rrr"]), @"Error: Invalid register: rrr");
        assert_snapshot!(cmd(&["r1", "r2", "123"]), @"Error: Invalid register: 123");

        assert_snapshot!(cmd(&["r1", "r2", "r3"]), @"0000 000 000 00001 00010 0000000 00011");
    }

    #[test]
    fn encode_i() {
        let cmd = instr("addi");
        assert_snapshot!(cmd(&["r1", "r2"]), @"Error: Instruction 'addi' requires 3 operands, got 2");
        assert_snapshot!(cmd(&["r1", "r2", "r3", "r4"]), @"Error: Instruction 'addi' requires 3 operands, got 4");
        assert_snapshot!(cmd(&["r1", "rrr", "123"]), @"Error: Invalid register: rrr");
        assert_snapshot!(cmd(&["r1", "r2", "r3"]), @"Error: Expected immediate, undefined identifier: r3");
        assert_snapshot!(cmd(&["r1", "r2", "0xFFF"]), @"Error: Immediate '4095' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r2", "0x7FF"]), @"0000 000 100 00001 00010 0111111 11111");
        assert_snapshot!(cmd(&["r1", "r2", "0xFFFF"]), @"Error: Immediate '65535' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r2", "0xFFFFFFFF"]), @"0000 000 100 00001 00010 1111111 11111");
        assert_snapshot!(cmd(&["r1", "r2", "-1"]), @"0000 000 100 00001 00010 1111111 11111");

        assert_snapshot!(cmd(&["r1", "r2", "3"]), @"0000 000 100 00001 00010 0000000 00011");
        assert_snapshot!(cmd(&["r1", "r2", "2047"]), @"0000 000 100 00001 00010 0111111 11111");
        assert_snapshot!(cmd(&["r1", "r2", "2048"]), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r2", "-3"]), @"0000 000 100 00001 00010 1111111 11101");
        assert_snapshot!(cmd(&["r1", "r2", "-2048"]), @"0000 000 100 00001 00010 1000000 00000");
        assert_snapshot!(cmd(&["r1", "r2", "-2049"]), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");

        let cmd = instr("srli");

        assert_snapshot!(cmd(&["r1", "r2", "32"]), @"Error: Immediate '32' out of range for u5 (0 ..= 31)");
        assert_snapshot!(cmd(&["r1", "r2", "31"]), @"0000 101 001 00001 00010 0000000 11111");
    }

    #[test]
    fn enocde_b() {
        let cmd = instr("beq");
        // Same to I-type, omitting ...
        assert_snapshot!(cmd(&["r1", "r0", "over"]), @"Error: Address offset '596523' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r0", "loop"]), @"0001 001 000 00000 00001 0000010 00000");

        let cmd = instr("sw");

        assert_snapshot!(cmd(&["r1", "r2", "3"]), @"0001 000 101 00000 00010 0000011 00001");
        assert_snapshot!(cmd(&["r1", "r2", "2047"]), @"0001 000 101 01111 00010 1111111 00001");
        assert_snapshot!(cmd(&["r1", "r2", "2048"]), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r2", "-3"]), @"0001 000 101 11111 00010 1111101 00001");
        assert_snapshot!(cmd(&["r1", "r2", "-2048"]), @"0001 000 101 10000 00010 0000000 00001");
        assert_snapshot!(cmd(&["r1", "r2", "-2049"]), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");
    }

    #[test]
    fn encode_u() {
        let cmd = instr("lui");

        assert_snapshot!(cmd(&["r1"]), @"Error: Instruction 'lui' requires 2 operands, got 1");
        assert_snapshot!(cmd(&["r1", "r2", "r3"]), @"Error: Instruction 'lui' requires 2 operands, got 3");
        assert_snapshot!(cmd(&["r1", "r2"]), @"Error: Expected immediate, undefined identifier: r2");
        assert_snapshot!(cmd(&["r3", "0x200000"]), @"Error: Immediate '2097152' out of range for u20 (0 ..= 1048575)");
        assert_snapshot!(cmd(&["r3", "-123"]), @"Error: Immediate '-123' out of range for u20 (0 ..= 1048575)");

        assert_snapshot!(cmd(&["r3", "0xABCDE"]), @"0001 011 101 00011 01011 1100110 11110");
    }
}
