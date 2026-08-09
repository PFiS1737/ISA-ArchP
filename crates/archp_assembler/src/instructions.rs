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
    operand::{OperandType, OperandValue, op_fmt},
    parser::{address::parse_address, immediate::parse_imm_as, register::parse_reg},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstrType {
    R,
    I,
    B,
    S,
    U,
    J,
}

inventory::collect!(&'static dyn Instruction);

pub static INSTRUCTIONS: LazyLock<HashMap<&'static str, &'static dyn Instruction>> =
    LazyLock::new(|| {
        let mut map = HashMap::new();
        for entry in inventory::iter::<&'static dyn Instruction> {
            map.insert(entry.name(), *entry);
        }
        map
    });

pub trait Instruction: Send + Sync {
    fn name(&self) -> &'static str;
    fn opcode(&self) -> u32;
    fn funct3(&self) -> u32;
    fn itype(&self) -> InstrType;
    fn operands_format(&self) -> Option<&'static [Option<OperandType>]>;

    fn encode(&self, ctx: &Context, pc: u32, operands: &[OperandValue]) -> Result<u32> {
        let operands = self.parse(ctx, pc, operands)?;

        match self.itype() {
            InstrType::R => self.encode_r(&operands),
            InstrType::I => self.encode_i(&operands),
            InstrType::B => self.encode_b(&operands),
            InstrType::S => self.encode_s(&operands),
            InstrType::U => self.encode_u(&operands),
            InstrType::J => self.encode_j(&operands),
        }
    }

    fn parse(&self, ctx: &Context, pc: u32, operands: &[OperandValue]) -> Result<Vec<u32>> {
        let format = self.get_operands_format();

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
                        OperandType::RegD | OperandType::RegS => parse_reg(ctx, op)?,
                        OperandType::Imm(bits, signed) => parse_imm_as(ctx, op, bits, signed)?,
                        OperandType::Addr(bits) => parse_address(ctx, op)?.as_field(bits, pc)?,
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
            self.opcode(), 25, 7;
            self.funct3(), 22, 3;
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
            self.opcode(), 25, 7;
            self.funct3(), 22, 3;
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
            self.opcode(), 25, 7;
            self.funct3(), 22, 3;
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
            self.opcode(), 25, 7;
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
                self.name(),
                expected,
                count
            );
        }

        Ok(())
    }

    fn get_operands_format(&self) -> &'static [Option<OperandType>] {
        if let Some(ops) = self.operands_format() {
            ops
        } else {
            match self.itype() {
                InstrType::R => op_fmt![RegD, RegS, RegS],
                InstrType::I => op_fmt![RegD, RegS, Imm(12, i)],
                InstrType::B => op_fmt![RegS, RegS, Addr(12)],
                InstrType::S => op_fmt![RegS, RegS, Imm(12, i)],
                InstrType::U => op_fmt![RegD, Imm(20, u)],
                InstrType::J => op_fmt![RegD, Addr(20)],
            }
        }
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

macro impl_instruction {
    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            funct3: $funct3:literal,
            itype: $itype:ident,
            operands_format: $opt:ident $( ($format:tt) )? ,
        }
    ) => {
        $( #[doc = $doc] )*
        $vis struct $id;

        impl $crate::instructions::Instruction for $id {
            fn name(&self) -> &'static str {
                $name
            }
            fn opcode(&self) -> u32 {
                $opcode
            }
            fn funct3(&self) -> u32 {
                $funct3
            }
            fn itype(&self) -> $crate::instructions::InstrType {
                $crate::instructions::InstrType::$itype
            }
            fn operands_format(&self) -> Option<&'static [ Option<$crate::operand::OperandType> ]> {
                $opt $( ( $crate::operand::op_fmt! $format ) )?
            }
        }

        inventory::submit! {
            &$id as &'static dyn $crate::instructions::Instruction
        }
    },
}

macro instruction {
    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
            funct3: $funct3:literal,
            itype: $itype:ident,
        }
    ) => {
        $crate::instructions::impl_instruction!{
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: $funct3,
                itype: $itype,
                operands_format: None,
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
        $crate::instructions::impl_instruction!{
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: 0,
                itype: $itype,
                operands_format: None,
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
        $crate::instructions::impl_instruction!{
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: $funct3,
                itype: $itype,
                operands_format: Some($format),
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
        $crate::instructions::impl_instruction!{
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
                funct3: 0,
                itype: $itype,
                operands_format: Some($format),
            }
        }
    },
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

#[cfg(test)]
mod tests {
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
        assert_snapshot!(cmd(&["r1", "r2", "r3"]), @"Error: Failed to evaluate immediate 'r3': unknown identifier: r3");
        assert_snapshot!(cmd(&["r1", "r2", "0xFFF"]), @"Error: Immediate '0xFFF' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r2", "0x7FF"]), @"0000 000 100 00001 00010 0111111 11111");
        assert_snapshot!(cmd(&["r1", "r2", "0xFFFF"]), @"Error: Immediate '0xFFFF' out of range for i12 (-2048 ..= 2047)");
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
        assert_snapshot!(cmd(&["r1", "r2"]), @"Error: Failed to evaluate immediate 'r2': unknown identifier: r2");
        assert_snapshot!(cmd(&["r3", "0x200000"]), @"Error: Immediate '0x200000' out of range for u20 (0 ..= 1048575)");
        assert_snapshot!(cmd(&["r3", "-123"]), @"Error: Immediate '-123' out of range for u20 (0 ..= 1048575)");

        assert_snapshot!(cmd(&["r3", "0xABCDE"]), @"0001 011 101 00011 01011 1100110 11110");
    }
}
