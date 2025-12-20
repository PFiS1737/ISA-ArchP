mod arithmetic;
mod branch;
mod call_ret;
mod compare;
mod display;
mod load_store;
mod logic;
mod shift;
mod stack;

use std::{collections::HashMap, fmt::Display};

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::{
    assembler::Context,
    operand::{OperandType, OperandValue, op_types},
    parser::{parse_cond, parse_imm, parse_reg_d, parse_reg_s},
};

#[derive(Debug, Clone, Copy, PartialEq)]
enum InstrType {
    R,
    I,
    B,
    U,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum FormatPlaceholder {
    None,
    Some,
}

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    name: &'static str,
    opcode: u32,
    itype: InstrType,
    operand_types: Option<&'static [OperandType]>,
    encode_format: Option<[FormatPlaceholder; 3]>,
}

inventory::collect!(Instruction);

pub static INSTRUCTIONS: Lazy<HashMap<&'static str, Instruction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for entry in inventory::iter::<Instruction> {
        map.insert(entry.name, *entry);
    }
    map
});

impl Instruction {
    pub fn encode(
        &self,
        ctx: &Context,
        cond: Option<&str>,
        operands: &[OperandValue],
    ) -> Result<u32> {
        let cond = cond.map(parse_cond).transpose()?.unwrap_or(0);

        if matches!(self.itype, InstrType::U | InstrType::C) && cond != 0 {
            bail!(
                "Condition is not allowed for {}-type instruction '{}'",
                self.itype,
                self.name
            );
        }

        let operands = self.parse(ctx, operands)?;

        match self.itype {
            InstrType::R => self.encode_r(cond, &operands),
            InstrType::I => self.encode_i(cond, &operands),
            InstrType::B => self.encode_b(cond, &operands),
            InstrType::U => self.encode_u(cond, &operands),
            InstrType::C => self.encode_c(cond, &operands),
        }
    }

    fn parse(&self, ctx: &Context, operands: &[OperandValue]) -> Result<Vec<u32>> {
        let mut parsed_operands = Vec::new();
        let operand_types = self.get_operand_types();

        self.assert_operand_count(operands.len(), operand_types.len())?;

        for (i, op) in operands.iter().enumerate() {
            match operand_types[i] {
                OperandType::RegD => {
                    let reg = parse_reg_d(ctx, op)?;
                    parsed_operands.push(reg);
                }
                OperandType::RegS => {
                    let reg = parse_reg_s(ctx, op)?;
                    parsed_operands.push(reg);
                }
                OperandType::Imm(bits, signed) => {
                    let imm = parse_imm(ctx, op)?.as_field(bits, signed)?;
                    parsed_operands.push(imm);
                }
            }
        }

        if let Some(format) = self.encode_format {
            if !matches!(self.itype, InstrType::R | InstrType::I | InstrType::B) {
                panic!(
                    "Internal Error: 'encode_format' is only supported for R/I/B-type instructions, foundinstruction '{}'",
                    self.name
                );
            }

            let mut formatted_operands = Vec::new();
            let mut operand_index = 0;

            for placeholder in format.iter() {
                match placeholder {
                    FormatPlaceholder::Some => {
                        formatted_operands.push(parsed_operands[operand_index]);
                        operand_index += 1;
                    }
                    FormatPlaceholder::None => {
                        formatted_operands.push(0);
                    }
                }
            }

            Ok(formatted_operands)
        } else {
            Ok(parsed_operands)
        }
    }

    // xxxx xxx   xxx   xxxxx   xxxxx   0000000   xxxxx
    //  opcode  | cond|   rd  |  rs1  |    --   |  rs2
    fn encode_r(&self, cond: u32, operands: &[u32]) -> Result<u32> {
        let rd = operands[0];
        let rs1 = operands[1];
        let rs2 = operands[2];

        code!(self.opcode, cond, rd, rs1, rs2)
    }

    // xxxx xxx   xxx   xxxxx   xxxxx   xxxxxxxxxxxx
    //  opcode  | cond|   rd  |  rs1  |    imm12
    fn encode_i(&self, cond: u32, operands: &[u32]) -> Result<u32> {
        let rd = operands[0];
        let rs1 = operands[1];
        let imm12 = operands[2];

        code!(self.opcode, cond, rd, rs1, imm12)
    }

    // 1001 xxx   xxx   xxxxx   xxxxx   xxxxxxx   xxxxx
    //  opcode  | cond|  up5  |  rs1  |   low7  |  rs2  (offset12 = up5 << 7 | low7)
    fn encode_b(&self, cond: u32, operands: &[u32]) -> Result<u32> {
        let rs1 = operands[0];
        let rs2 = operands[1];
        let offset12 = operands[2];

        code!(
            self.opcode,
            cond,
            (offset12 >> 7),
            rs1,
            (offset12 & 0x7F),
            rs2
        )
    }

    // 1000 100   xxx   xxxxx   xxxxxxxxxxxxxxxxx
    //    lui  |uimm20u|  rd  |      uimm20l      (uimm20 = uimm20u << 17 | uimm20l)
    fn encode_u(&self, _: u32, operands: &[u32]) -> Result<u32> {
        let rd = operands[0];
        let imm20 = operands[1];

        code!(self.opcode, (imm20 >> 17), rd, (imm20 & 0x1FFFF))
    }

    // 1101 000   0   xxxxxxxx xxxxxxxx xxxxxxxx
    //    col   | - |           color24
    fn encode_c(&self, _: u32, operands: &[u32]) -> Result<u32> {
        let color24 = operands[0];

        code!(self.opcode, 0, 0, color24)
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

    fn get_operand_types(&self) -> &'static [OperandType] {
        if let Some(ops) = self.operand_types {
            ops
        } else {
            match self.itype {
                InstrType::R => op_types![RegD, RegS, RegS],
                InstrType::I => op_types![RegD, RegS, Imm(12, i)],
                InstrType::B => op_types![RegS, RegS, Imm(12, u)],
                InstrType::U => op_types![RegD, Imm(20, u)],
                InstrType::C => op_types![Imm(24, u)],
            }
        }
    }
}

macro code {
    // R/I-type
    ($opcode:expr, $cond:expr, $rd:expr, $rs1:expr, $rs2_or_imm12:expr) => {
        Ok(($opcode << 25) | ($cond << 22) | ($rd << 17) | ($rs1 << 12) | $rs2_or_imm12)
    },

    // B-type
    ($opcode:expr, $cond:expr, $up5:expr, $rs1:expr, $low7:expr, $rs2:expr) => {
        Ok(($opcode << 25) | ($cond << 22) | ($up5 << 17) | ($rs1 << 12) | ($low7 << 5) | $rs2)
    },

    // U/C-type
    ($opcode:expr, $uimm20u:expr, $rd:expr, $uimm20l:expr) => {
        Ok(($opcode << 25) | ($uimm20u << 22) | ($rd << 17) | $uimm20l)
    },
}

macro instruction {
    (
        name: $name:literal,
        opcode: $opcode:literal,
        itype: $itype:ident,
    ) => {
        inventory::submit! {
            $crate::instructions::Instruction {
                name: $name,
                opcode: $opcode,
                itype: $crate::instructions::InstrType::$itype,
                operand_types: None,
                encode_format: None,
            }
        }
    },

    (
        name: $name:literal,
        opcode: $opcode:literal,
        itype: $itype:ident,
        operand_types: $types:tt,
    ) => {
        inventory::submit! {
            $crate::instructions::Instruction {
                name: $name,
                opcode: $opcode,
                itype: $crate::instructions::InstrType::$itype,
                operand_types: Some($crate::operand::op_types! $types),
                encode_format: None,
            }
        }
    },

    (
        name: $name:literal,
        opcode: $opcode:literal,
        itype: $itype:ident,
        operand_types: $types:tt,
        encode_format: [ $rd:ident, $rs1:ident, $rs2:ident ],
    ) => {
        inventory::submit! {
            $crate::instructions::Instruction {
                name: $name,
                opcode: $opcode,
                itype: $crate::instructions::InstrType::$itype,
                operand_types: Some($crate::operand::op_types! $types),
                encode_format: Some([
                    $crate::instructions::FormatPlaceholder::$rd,
                    $crate::instructions::FormatPlaceholder::$rs1,
                    $crate::instructions::FormatPlaceholder::$rs2
                ]),
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
            InstrType::U => write!(f, "U"),
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

        assert_snapshot!(cmd("", &["r1", "r2"]), @"Error: Instruction 'add' requires 3 operands, got 2");
        assert_snapshot!(cmd("", &["r1", "r2", "r3", "r4"]), @"Error: Instruction 'add' requires 3 operands, got 4");
        assert_snapshot!(cmd("", &["r1", "r2", "rrr"]), @"Error: Invalid register: rrr");
        assert_snapshot!(cmd("", &["r1", "r2", "123"]), @"Error: Expected register, found immediate: 123");
        assert_snapshot!(cmd("", &["r0", "r2", "r3"]), @"Error: Register 'r0' is raed-only");
        assert_snapshot!(cmd("invalid", &["r1", "r2", "r3"]), @"Error: Invalid condition: invalid");

        assert_snapshot!(cmd("lt", &["r1", "r2", "r3"]), @"0000 000 011 00001 00010 0000000 00011");
    }

    #[test]
    fn encode_i() {
        let cmd = instr("addi");
        assert_snapshot!(cmd("", &["r1", "r2"]), @"Error: Instruction 'addi' requires 3 operands, got 2");
        assert_snapshot!(cmd("", &["r1", "r2", "r3", "r4"]), @"Error: Instruction 'addi' requires 3 operands, got 4");
        assert_snapshot!(cmd("", &["r1", "rrr", "123"]), @"Error: Invalid register: rrr");
        assert_snapshot!(cmd("", &["r1", "r2", "r3"]), @"Error: Invalid immediate: r3");
        assert_snapshot!(cmd("", &["r0", "r2", "123"]), @"Error: Register 'r0' is raed-only");
        assert_snapshot!(cmd("", &["r1", "r2", "0xFFFF"]), @"Error: Immediate '65535' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd("invalid", &["r1", "r2", "123"]), @"Error: Invalid condition: invalid");

        assert_snapshot!(cmd("", &["r1", "r2", "3"]), @"0100 000 000 00001 00010 0000000 00011");
        assert_snapshot!(cmd("", &["r1", "r2", "2047"]), @"0100 000 000 00001 00010 0111111 11111");
        assert_snapshot!(cmd("", &["r1", "r2", "2048"]), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd("", &["r1", "r2", "-3"]), @"0100 000 000 00001 00010 1111111 11101");
        assert_snapshot!(cmd("", &["r1", "r2", "-2048"]), @"0100 000 000 00001 00010 1000000 00000");
        assert_snapshot!(cmd("", &["r1", "r2", "-2049"]), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");

        let cmd = instr("shri");

        assert_snapshot!(cmd("", &["r1", "r2", "32"]), @"Error: Immediate '32' out of range for u5 (0 ..= 31)");
        assert_snapshot!(cmd("", &["r1", "r2", "31"]), @"0110 001 000 00001 00010 0000000 11111");

        let cmd = instr("li");

        assert_snapshot!(cmd("", &["r1", "3"]), @"1000 010 000 00001 00000 0000000 00011");
        assert_snapshot!(cmd("", &["r1", "2047"]), @"1000 010 000 00001 00000 0111111 11111");
        assert_snapshot!(cmd("", &["r1", "2048"]), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd("", &["r1", "-3"]), @"1000 010 000 00001 00000 1111111 11101");
        assert_snapshot!(cmd("", &["r1", "-2048"]), @"1000 010 000 00001 00000 1000000 00000");
        assert_snapshot!(cmd("", &["r1", "-2049"]), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");
    }

    #[test]
    fn enocde_b() {
        let cmd = instr("beq");
        // Same to I-type, omitting ...
        assert_snapshot!(cmd("ne", &["r1", "r0", "-1"]), @"Error: Immediate '-1' out of range for u12 (must be >= 0)");
        assert_snapshot!(cmd("ne", &["r1", "r0", "3456"]), @"1001 001 010 11011 00001 0000000 00000");

        let cmd = instr("sw");

        assert_snapshot!(cmd("", &["r1", "r2", "3"]), @"1000 001 000 00000 00001 0000011 00010");
        assert_snapshot!(cmd("", &["r1", "r2", "2047"]), @"1000 001 000 01111 00001 1111111 00010");
        assert_snapshot!(cmd("", &["r1", "r2", "2048"]), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(cmd("", &["r1", "r2", "-3"]), @"1000 001 000 11111 00001 1111101 00010");
        assert_snapshot!(cmd("", &["r1", "r2", "-2048"]), @"1000 001 000 10000 00001 0000000 00010");
        assert_snapshot!(cmd("", &["r1", "r2", "-2049"]), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");
    }

    #[test]
    fn encode_u() {
        let cmd = instr("lui");

        assert_snapshot!(cmd("", &["r1"]), @"Error: Instruction 'lui' requires 2 operands, got 1");
        assert_snapshot!(cmd("", &["r1", "r2", "r3"]), @"Error: Instruction 'lui' requires 2 operands, got 3");
        assert_snapshot!(cmd("", &["r1", "r2"]), @"Error: Invalid immediate: r2");
        assert_snapshot!(cmd("", &["r0", "r2"]), @"Error: Register 'r0' is raed-only");
        assert_snapshot!(cmd("", &["r3", "0x200000"]), @"Error: Immediate '2097152' out of range for u20 (0 ..= 1048575)");
        assert_snapshot!(cmd("", &["r3", "-123"]), @"Error: Immediate '-123' out of range for u20 (must be >= 0)");
        assert_snapshot!(cmd("eq", &["r3", "0xABCDE"]), @"Error: Condition is not allowed for U-type instruction 'lui'");

        assert_snapshot!(cmd("", &["r3", "0xABCDE"]), @"1000 011 101 00011 01011 1100110 11110");
    }

    #[test]
    fn encode_c() {
        let cmd = instr("col");

        assert_snapshot!(cmd("", &[]), @"Error: Instruction 'col' requires 1 operands, got 0");
        assert_snapshot!(cmd("", &["r1", "r2"]), @"Error: Instruction 'col' requires 1 operands, got 2");
        assert_snapshot!(cmd("", &["r1"]), @"Error: Invalid immediate: r1");
        assert_snapshot!(cmd("", &["0x1FFFFFF"]), @"Error: Immediate '33554431' out of range for u24 (0 ..= 16777215)");
        assert_snapshot!(cmd("", &["-123"]), @"Error: Immediate '-123' out of range for u24 (must be >= 0)");
        assert_snapshot!(cmd("ne", &["0x123456"]), @"Error: Condition is not allowed for C-type instruction 'col'");

        assert_snapshot!(cmd("", &["0x123456"]), @"1101 000 000 01001 00011 0100010 10110");
    }
}
