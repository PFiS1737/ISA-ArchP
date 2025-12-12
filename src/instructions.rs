mod arithmetic;
mod branch;
mod call_ret;
mod compare;
mod load_store;
mod logic;
mod pixel;
mod random;
mod shift;
mod stack;

use std::{collections::HashMap, fmt::Display, num::IntErrorKind};

use anyhow::{Result, anyhow, bail};
use once_cell::sync::Lazy;

use crate::operand::{ImmRange, OperandType, OperandValue, op_types};

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
    pub fn encode(&self, cond: Option<&str>, operands: &[OperandValue]) -> Result<u32> {
        let cond = cond.map(parse_cond).transpose()?.unwrap_or(0);

        if matches!(self.itype, InstrType::U | InstrType::C) && cond != 0 {
            bail!(
                "Condition is not allowed for {}-type instruction '{}'",
                self.itype,
                self.name
            );
        }

        let operands = self.parse(operands)?;

        match self.itype {
            InstrType::R => self.encode_r(cond, &operands),
            InstrType::I => self.encode_i(cond, &operands),
            InstrType::B => self.encode_b(cond, &operands),
            InstrType::U => self.encode_u(cond, &operands),
            InstrType::C => self.encode_c(cond, &operands),
        }
    }

    fn parse(&self, operands: &[OperandValue]) -> Result<Vec<u32>> {
        let mut parsed_operands = Vec::new();
        let operand_types = self.get_operand_types();

        self.assert_operand_count(operands.len(), operand_types.len())?;

        for (i, op) in operands.iter().enumerate() {
            match &operand_types[i] {
                OperandType::RegD => {
                    let reg = parse_reg_d(op)?;
                    parsed_operands.push(reg);
                }
                OperandType::RegS => {
                    let reg = parse_reg_s(op)?;
                    parsed_operands.push(reg);
                }
                OperandType::Imm(range) => {
                    let imm = parse_imm(op)?;

                    self.assert_immediate_range(imm, range)?;

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

    fn assert_immediate_range(&self, imm: u32, range: &ImmRange) -> Result<()> {
        if !range.contains(&imm) {
            bail!(
                "Immediate value '{}' out of range for {}-type instruction '{}', expected {}",
                imm,
                self.itype,
                self.name,
                range
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
                InstrType::I => op_types![RegD, RegS, Imm(12)],
                InstrType::B => op_types![RegS, RegS, Imm(12)],
                InstrType::U => op_types![RegD, Imm(20)],
                InstrType::C => op_types![Imm(24)],
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

fn parse_cond(cond: &str) -> Result<u32> {
    match cond {
        "eq" => Ok(0b001),
        "ne" => Ok(0b010),
        "lt" => Ok(0b011),
        "ge" => Ok(0b100),
        "gt" => Ok(0b101),
        "le" => Ok(0b110),
        _ => bail!("Invalid condition: {}", cond),
    }
}

macro err_expect_reg($e:expr) {
    bail!("Expected register, found immediate: {}", $e)
}
macro err_inval_reg($e:expr) {
    bail!("Invalid register: {}", $e)
}
macro err_reg_out_of_range($e:expr, $s:expr) {
    bail!("Register number out of range ({}-24): {}", $s, $e)
}
macro err_read_only_reg($e:expr) {
    bail!("Register '{}' is raed-only", $e)
}

pub fn parse_reg_d(op: &OperandValue) -> Result<u32> {
    let reg = match op {
        OperandValue::StringSlice(s) => *s,
        OperandValue::Unsigned(n) => err_expect_reg!(n),
    };

    match reg {
        "io" => Ok(26),
        "tmp" => Ok(31),

        "zero" | "pc" | "kb" => err_read_only_reg!(reg),

        r if let Some(n) = r.strip_prefix("r")
            && let Ok(n) = n.parse::<u32>() =>
        {
            if n > 24 {
                err_reg_out_of_range!(reg, "1");
            }
            Ok(n)
        }

        _ => {
            if parse_imm(op).is_ok() {
                err_expect_reg!(reg)
            } else {
                err_inval_reg!(reg)
            }
        }
    }
}

pub fn parse_reg_s(op: &OperandValue) -> Result<u32> {
    let reg = match op {
        OperandValue::StringSlice(s) => *s,
        OperandValue::Unsigned(n) => err_expect_reg!(n),
    };

    match reg {
        "zero" => Ok(0),

        "pc" => Ok(25),
        "io" => Ok(26),
        "kb" => Ok(27),
        "tmp" => Ok(31),

        r if let Some(n) = r.strip_prefix("r")
            && let Ok(n) = n.parse::<u32>() =>
        {
            if n > 24 {
                err_reg_out_of_range!(reg, "0");
            }
            Ok(n)
        }

        _ => {
            if parse_imm(op).is_ok() {
                err_expect_reg!(reg)
            } else {
                err_inval_reg!(reg)
            }
        }
    }
}

pub fn parse_imm(imm: &OperandValue) -> Result<u32> {
    let parse_str = |s: &str| match s {
        s if let Some(hex) = s.strip_prefix("0x") => u32::from_str_radix(hex, 16),
        s if let Some(bin) = s.strip_prefix("0b") => u32::from_str_radix(bin, 2),
        s => s.parse(),
    };

    let parsed = match imm {
        OperandValue::StringSlice(s) => parse_str(s),
        OperandValue::Unsigned(n) => Ok(*n),
    };

    parsed.map_err(|err| {
        if err.kind() == &IntErrorKind::PosOverflow {
            anyhow!("Immediate out of range of 32-bits: {}", imm)
        } else {
            anyhow!("Invalid immediate: {}", imm)
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::{operand::OperandValue, testkit::*};
    use anyhow::Result;

    // WARN: 也许这里不适合用快照测试?
    // TODO: 以后如果有迁移到 thiserror 的打算，再回来改

    #[test]
    fn parse_cond() {
        let f = |s| match super::parse_cond(s) {
            Ok(n) => format!("{n}"),
            Err(e) => format!("Error: {e}"),
        };
        assert_snapshot!(f("eq"), @"1");
        assert_snapshot!(f("ne"), @"2");
        assert_snapshot!(f("lt"), @"3");
        assert_snapshot!(f("ge"), @"4");
        assert_snapshot!(f("gt"), @"5");
        assert_snapshot!(f("le"), @"6");
        assert_snapshot!(f("invalid"), @"Error: Invalid condition: invalid");
    }

    fn test(func: fn(&OperandValue) -> Result<u32>) -> impl Fn(&str) -> String {
        move |s| match func(&OperandValue::from(s)) {
            Ok(n) => format!("{n}"),
            Err(e) => format!("Error: {e}"),
        }
    }

    #[test]
    fn parse_reg_d() {
        let f = test(super::parse_reg_d);
        assert_snapshot!(f("zero"), @"Error: Register 'zero' is raed-only");
        assert_snapshot!(f("r9"), @"9");
        assert_snapshot!(f("r27"), @"Error: Register number out of range (1-24): r27");
        assert_snapshot!(f("kb"), @"Error: Register 'kb' is raed-only");
        assert_snapshot!(f("invalid"), @"Error: Invalid register: invalid");
    }

    #[test]
    fn parse_reg_s() {
        let f = test(super::parse_reg_s);
        assert_snapshot!(f("zero"), @"0");
        assert_snapshot!(f("r15"), @"15");
        assert_snapshot!(f("r30"), @"Error: Register number out of range (0-24): r30");
        assert_snapshot!(f("pc"), @"25");
        assert_snapshot!(f("invalid"), @"Error: Invalid register: invalid");
    }

    #[test]
    fn parse_imm() {
        let f = test(super::parse_imm);
        assert_snapshot!(f("42"), @"42");
        assert_snapshot!(f("0x2A"), @"42");
        assert_snapshot!(f("0b101010"), @"42");
        assert_snapshot!(f("r1"), @"Error: Invalid immediate: r1");
        assert_snapshot!(f("invalid"), @"Error: Invalid immediate: invalid");
        assert_snapshot!(f("0x1FFFFFFFF"), @"Error: Immediate out of range of 32-bits: 0x1FFFFFFFF");
    }

    #[test]
    fn encode_r() {
        let cmd = instr("add");
        assert_snapshot!(cmd("", &["r1", "r2"]), @"Error: Instruction 'add' requires 3 operands, got 2");
        assert_snapshot!(cmd("", &["r1", "r2", "r3", "r4"]), @"Error: Instruction 'add' requires 3 operands, got 4");
        assert_snapshot!(cmd("", &["r1", "r2", "rrr"]), @"Error: Invalid register: rrr");
        assert_snapshot!(cmd("", &["r1", "r2", "123"]), @"Error: Expected register, found immediate: 123");
        assert_snapshot!(cmd("", &["zero", "r2", "r3"]), @"Error: Register 'zero' is raed-only");
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
        assert_snapshot!(cmd("", &["zero", "r2", "123"]), @"Error: Register 'zero' is raed-only");
        assert_snapshot!(cmd("", &["r1", "r2", "0xFFFF"]), @"Error: Immediate value '65535' out of range for I-type instruction 'addi', expected 0 ~ 0xFFF");
        assert_snapshot!(cmd("invalid", &["r1", "r2", "123"]), @"Error: Invalid condition: invalid");
        assert_snapshot!(cmd("ge", &["r4", "r5", "0b100"]), @"0100 000 100 00100 00101 0000000 00100");

        let cmd = instr("shri");
        assert_snapshot!(cmd("", &["r1", "r2", "32"]), @"Error: Immediate value '32' out of range for I-type instruction 'shri', expected 0 ~ 31");
        assert_snapshot!(cmd("", &["r1", "r2", "31"]), @"0110 001 000 00001 00010 0000000 11111");
    }

    #[test]
    fn enocde_b() {
        let cmd = instr("beq");
        // Same to I-type, omitting ...
        assert_snapshot!(cmd("ne", &["r1", "zero", "3456"]), @"1001 001 010 11011 00001 0000000 00000");
    }

    #[test]
    fn encode_u() {
        let cmd = instr("lui");
        assert_snapshot!(cmd("", &["r1"]), @"Error: Instruction 'lui' requires 2 operands, got 1");
        assert_snapshot!(cmd("", &["r1", "r2", "r3"]), @"Error: Instruction 'lui' requires 2 operands, got 3");
        assert_snapshot!(cmd("", &["r1", "r2"]), @"Error: Invalid immediate: r2");
        assert_snapshot!(cmd("", &["zero", "r2"]), @"Error: Register 'zero' is raed-only");
        assert_snapshot!(cmd("", &["r3", "0x200000"]), @"Error: Immediate value '2097152' out of range for U-type instruction 'lui', expected 0 ~ 0xFFFFF");
        assert_snapshot!(cmd("eq", &["r3", "0xABCDE"]), @"Error: Condition is not allowed for U-type instruction 'lui'");
        assert_snapshot!(cmd("", &["r3", "0xABCDE"]), @"1000 011 101 00011 01011 1100110 11110");
    }

    #[test]
    fn encode_c() {
        let cmd = instr("col");
        assert_snapshot!(cmd("", &[]), @"Error: Instruction 'col' requires 1 operands, got 0");
        assert_snapshot!(cmd("", &["r1", "r2"]), @"Error: Instruction 'col' requires 1 operands, got 2");
        assert_snapshot!(cmd("", &["r1"]), @"Error: Invalid immediate: r1");
        assert_snapshot!(cmd("", &["0x1FFFFFF"]), @"Error: Immediate value '33554431' out of range for C-type instruction 'col', expected 0 ~ 0xFFFFFF");
        assert_snapshot!(cmd("ne", &["0x123456"]), @"Error: Condition is not allowed for C-type instruction 'col'");
        assert_snapshot!(cmd("", &["0x123456"]), @"1101 000 000 01001 00011 0100010 10110");
    }
}
