mod arithmetic;
mod branch;
mod display;
mod jump_and_link;
mod load_store;
mod logic;
mod set;
mod shift;
mod stack_call_return;
mod upper_imm;

use std::{collections::HashMap, fmt::Display};

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::{
    assembler::Context,
    operand::{OperandType, OperandValue, op_fmt},
    parser::{parse_address, parse_imm, parse_reg_d, parse_reg_s},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstrType {
    R,
    I,
    B,
    S,
    U,
    J,
    C,
}

inventory::collect!(&'static dyn Instruction);

pub static INSTRUCTIONS: Lazy<HashMap<&'static str, &'static dyn Instruction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for entry in inventory::iter::<&'static dyn Instruction> {
        map.insert(entry.name(), *entry);
    }
    map
});

pub trait Instruction: Send + Sync {
    fn name(&self) -> &'static str;
    fn opcode(&self) -> u32;
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
            InstrType::C => self.encode_c(&operands),
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
                        OperandType::RegD => parse_reg_d(ctx, op)?,
                        OperandType::RegS => parse_reg_s(ctx, op)?,
                        OperandType::Imm(bits, signed) => {
                            parse_imm(ctx, op)?.as_field(bits, signed)?
                        },
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

    // xxxx xxx   000   xxxxx   xxxxx   0000000   xxxxx
    //  opcode  |  -  |   rd  |  rs1  |    --   |  rs2
    fn encode_r(&self, operands: &[u32]) -> Result<u32> {
        let rd = operands[0];
        let rs1 = operands[1];
        let rs2 = operands[2];

        code!(@R, self.opcode(), rd, rs1, rs2)
    }

    // xxxx xxx   000   xxxxx   xxxxx   xxxxxxxxxxxx
    //  opcode  |  -  |   rd  |  rs1  |    imm12
    fn encode_i(&self, operands: &[u32]) -> Result<u32> {
        let rd = operands[0];
        let rs1 = operands[1];
        let imm12 = operands[2];

        code!(@R, self.opcode(), rd, rs1, imm12)
    }

    // 1001 xxx   000    xxxxx   xxxxx   xxxxxxx   xxxxx
    //  opcode  |  -  |  up5  |  rs1  |   low7  |  rs2  (offset12 = up5 << 7 | low7)
    fn encode_b(&self, operands: &[u32]) -> Result<u32> {
        let rs1 = operands[0];
        let rs2 = operands[1];
        let offset12 = operands[2];

        code!(@B, self.opcode(), (offset12 >> 7), rs1, (offset12 & 0x7F), rs2)
    }
    fn encode_s(&self, operands: &[u32]) -> Result<u32> {
        self.encode_b(operands)
    }

    // 1000 100   xxx   xxxxx   xxxxxxxxxxxxxxxxx
    //    lui  |uimm20u|  rd  |      uimm20l      (uimm20 = uimm20u << 17 | uimm20l)
    fn encode_u(&self, operands: &[u32]) -> Result<u32> {
        let rd = operands[0];
        let imm20 = operands[1];

        code!(@U, self.opcode(), (imm20 >> 17), rd, (imm20 & 0x1FFFF))
    }
    fn encode_j(&self, operands: &[u32]) -> Result<u32> {
        self.encode_u(operands)
    }

    // 1101 000   0   xxxxxxxx xxxxxxxx xxxxxxxx
    //    col   | - |           color24
    fn encode_c(&self, operands: &[u32]) -> Result<u32> {
        let color24 = operands[0];

        code!(@U, self.opcode(), 0, 0, color24)
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
                InstrType::C => op_fmt![Imm(24, u)],
            }
        }
    }
}

macro code {
    // R/I-type
    (@R, $opcode:expr, $rd:expr, $rs1:expr, $rs2_or_imm12:expr) => {
        Ok(($opcode << 25) | ($rd << 17) | ($rs1 << 12) | $rs2_or_imm12)
    },

    // B-type
    (@B, $opcode:expr, $up5:expr, $rs1:expr, $low7:expr, $rs2:expr) => {
        Ok(($opcode << 25) | ($up5 << 17) | ($rs1 << 12) | ($low7 << 5) | $rs2)
    },

    // U/C-type
    (@U, $opcode:expr, $uimm20u:expr, $rd:expr, $uimm20l:expr) => {
        Ok(($opcode << 25) | ($uimm20u << 22) | ($rd << 17) | $uimm20l)
    },
}

macro impl_instruction {
    (
        $( #[doc = $doc:literal] )*
        $vis:vis $id:ident {
            name: $name:literal,
            opcode: $opcode:literal,
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
            itype: $itype:ident,
        }
    ) => {
        $crate::instructions::impl_instruction!{
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
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
            operands_format: $format:tt,
        }
    ) => {
        $crate::instructions::impl_instruction!{
            $( #[doc = $doc] )*
            $vis $id {
                name: $name,
                opcode: $opcode,
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
            InstrType::C => write!(f, "C"),
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
        assert_snapshot!(cmd(&["r1", "r2", "123"]), @"Error: Expected register, found immediate: 123");

        assert_snapshot!(cmd(&["r1", "r2", "r3"]), @"0000 000 000 00001 00010 0000000 00011");
    }

    #[test]
    fn encode_i() {
        let cmd = instr("addi");
        assert_snapshot!(cmd(&["r1", "r2"]), @"Error: Instruction 'addi' requires 3 operands, got 2");
        assert_snapshot!(cmd(&["r1", "r2", "r3", "r4"]), @"Error: Instruction 'addi' requires 3 operands, got 4");
        assert_snapshot!(cmd(&["r1", "rrr", "123"]), @"Error: Invalid register: rrr");
        assert_snapshot!(cmd(&["r1", "r2", "r3"]), @"Error: Invalid immediate: r3");
        assert_snapshot!(cmd(&["r1", "r2", "0xFFFF"]), @"Error: Immediate '65535' out of range for i12 (-2048 ..= 2047)");

        assert_snapshot!(cmd(&["r1", "r2", "3"]), @"0100 000 000 00001 00010 0000000 00011");
        assert_snapshot!(cmd(&["r1", "r2", "2047"]), @"0100 000 000 00001 00010 0111111 11111");
        assert_snapshot!(cmd(&["r1", "r2", "2048"]), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r2", "-3"]), @"0100 000 000 00001 00010 1111111 11101");
        assert_snapshot!(cmd(&["r1", "r2", "-2048"]), @"0100 000 000 00001 00010 1000000 00000");
        assert_snapshot!(cmd(&["r1", "r2", "-2049"]), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");

        let cmd = instr("srli");

        assert_snapshot!(cmd(&["r1", "r2", "32"]), @"Error: Immediate '32' out of range for u5 (0 ..= 31)");
        assert_snapshot!(cmd(&["r1", "r2", "31"]), @"0110 001 000 00001 00010 0000000 11111");
    }

    #[test]
    fn enocde_b() {
        let cmd = instr("beq");
        // Same to I-type, omitting ...
        assert_snapshot!(cmd(&["r1", "r0", "over"]), @"Error: Address offset '596523' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r0", "loop"]), @"1001 001 000 00000 00001 0000010 00000");

        let cmd = instr("sw");

        assert_snapshot!(cmd(&["r1", "r2", "3"]), @"1000 101 000 00000 00001 0000011 00010");
        assert_snapshot!(cmd(&["r1", "r2", "2047"]), @"1000 101 000 01111 00001 1111111 00010");
        assert_snapshot!(cmd(&["r1", "r2", "2048"]), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd(&["r1", "r2", "-3"]), @"1000 101 000 11111 00001 1111101 00010");
        assert_snapshot!(cmd(&["r1", "r2", "-2048"]), @"1000 101 000 10000 00001 0000000 00010");
        assert_snapshot!(cmd(&["r1", "r2", "-2049"]), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");
    }

    #[test]
    fn encode_u() {
        let cmd = instr("lui");

        assert_snapshot!(cmd(&["r1"]), @"Error: Instruction 'lui' requires 2 operands, got 1");
        assert_snapshot!(cmd(&["r1", "r2", "r3"]), @"Error: Instruction 'lui' requires 2 operands, got 3");
        assert_snapshot!(cmd(&["r1", "r2"]), @"Error: Invalid immediate: r2");
        assert_snapshot!(cmd(&["r3", "0x200000"]), @"Error: Immediate '2097152' out of range for u20 (0 ..= 1048575)");
        assert_snapshot!(cmd(&["r3", "-123"]), @"Error: Immediate '18446744073709551493' out of range for u20 (0 ..= 1048575)");

        assert_snapshot!(cmd(&["r3", "0xABCDE"]), @"1011 000 101 00011 01011 1100110 11110");
    }

    #[test]
    fn encode_c() {
        let cmd = instr("col");

        assert_snapshot!(cmd(&[]), @"Error: Instruction 'col' requires 1 operands, got 0");
        assert_snapshot!(cmd(&["r1", "r2"]), @"Error: Instruction 'col' requires 1 operands, got 2");
        assert_snapshot!(cmd(&["r1"]), @"Error: Invalid immediate: r1");
        assert_snapshot!(cmd(&["0x1FFFFFF"]), @"Error: Immediate '33554431' out of range for u24 (0 ..= 16777215)");
        assert_snapshot!(cmd(&["-123"]), @"Error: Immediate '18446744073709551493' out of range for u24 (0 ..= 16777215)");

        assert_snapshot!(cmd(&["0x123456"]), @"1101 000 000 01001 00011 0100010 10110");
    }
}
