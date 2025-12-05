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

use std::{collections::HashMap, fmt::Display};

use anyhow::{Result, anyhow, bail};
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Copy, PartialEq)]
enum InstrType {
    R,
    I,
    B,
    U,
    C,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum OperandType {
    Reg,
    Imm(u8), // bits
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

macro_rules! code {
    // R/I-type
    ($opcode:expr, $cond:expr, $rd:expr, $rs1:expr, $rs2_or_imm12:expr) => {
        Ok(($opcode << 25) | ($cond << 22) | ($rd << 17) | ($rs1 << 12) | $rs2_or_imm12)
    };

    // B-type
    ($opcode:expr, $cond:expr, $up5:expr, $rs1:expr, $low7:expr, $rs2:expr) => {
        Ok(($opcode << 25) | ($cond << 22) | ($up5 << 17) | ($rs1 << 12) | ($low7 << 5) | $rs2)
    };

    // U/C-type
    ($opcode:expr, $uimm20u:expr, $rd:expr, $uimm20l:expr) => {
        Ok(($opcode << 25) | ($uimm20u << 22) | ($rd << 17) | $uimm20l)
    };
}

impl Instruction {
    pub fn encode(&self, cond: Option<&str>, operands: &[&str]) -> Result<u32> {
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

    fn parse(&self, operands: &[&str]) -> Result<Vec<u32>> {
        let mut parsed_operands = Vec::new();
        let operand_types = self.get_operand_types();

        self.assert_operand_count(operands.len(), operand_types.len())?;

        for (i, operand_str) in operands.iter().enumerate() {
            match operand_types[i] {
                OperandType::Reg => {
                    let reg = parse_reg(operand_str)?;
                    parsed_operands.push(reg);
                }
                OperandType::Imm(bits) => {
                    let imm = parse_imm(operand_str)?;

                    self.assert_immediate_range(imm, bits)?;

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

    fn assert_immediate_range(&self, imm: u32, bits: u8) -> Result<()> {
        if imm >= (1 << bits) {
            bail!(
                "Immediate value '{}' out of range for {}-type instruction '{}', expected {} bits",
                imm,
                self.itype,
                self.name,
                bits
            );
        }

        Ok(())
    }

    fn get_operand_types(&self) -> &'static [OperandType] {
        if let Some(ops) = self.operand_types {
            ops
        } else {
            match self.itype {
                InstrType::R => &[OperandType::Reg, OperandType::Reg, OperandType::Reg],
                InstrType::I => &[OperandType::Reg, OperandType::Reg, OperandType::Imm(12)],
                InstrType::B => &[OperandType::Reg, OperandType::Reg, OperandType::Imm(12)],
                InstrType::U => &[OperandType::Reg, OperandType::Imm(20)],
                InstrType::C => &[OperandType::Imm(24)],
            }
        }
    }
}

#[macro_export]
macro_rules! instruction {
    (
        name: $name:expr,
        opcode: $opcode:expr,
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
    };

    (
        name: $name:expr,
        opcode: $opcode:expr,
        itype: $itype:ident,
        operand_types: [ $( $operand_type:ident $(($v:expr))? ),* ],
        encode_format: [ $rd:ident, $rs1:ident, $rs2:ident ],
    ) => {
        inventory::submit! {
            $crate::instructions::Instruction {
                name: $name,
                opcode: $opcode,
                itype: $crate::instructions::InstrType::$itype,
                operand_types: Some(&[ $( $crate::instructions::OperandType::$operand_type $(($v))? ),* ]),
                encode_format: Some([
                    $crate::instructions::FormatPlaceholder::$rd,
                    $crate::instructions::FormatPlaceholder::$rs1,
                    $crate::instructions::FormatPlaceholder::$rs2
                ]),
            }
        }
    };
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
        "le" => Ok(0b100),
        "gt" => Ok(0b101),
        "ge" => Ok(0b110),
        _ => bail!("Invalid condition: {}", cond),
    }
}

pub fn parse_reg(reg: &str) -> Result<u32> {
    match reg {
        "r0" => Ok(0),
        "r1" => Ok(1),
        "r2" => Ok(2),
        "r3" => Ok(3),
        "r4" => Ok(4),
        "r5" => Ok(5),
        "s0" => Ok(6),
        "s1" => Ok(7),
        "s2" => Ok(8),
        "s3" => Ok(9),
        "s4" => Ok(10),
        "s5" => Ok(11),
        "t0" => Ok(12),
        "t1" => Ok(13),
        "t2" => Ok(14),
        "t3" => Ok(15),
        "t4" => Ok(16),
        "t5" => Ok(17),
        "a0" => Ok(18),
        "a1" => Ok(19),
        "a2" => Ok(20),
        "a3" => Ok(21),
        "a4" => Ok(22),
        "a5" => Ok(23),
        // "pc" => Ok(24),
        // "io" => Ok(25),
        "kb" => Ok(26),
        _ => {
            if parse_imm(reg).is_ok() {
                bail!("Expected register, found immediate: {}", reg)
            } else {
                bail!("Invalid register: {}", reg)
            }
        }
    }
}

pub fn parse_imm(imm: &str) -> Result<u32> {
    let parsed = match imm {
        s if let Some(hex) = s.strip_prefix("0x") => u32::from_str_radix(hex, 16),
        s if let Some(bin) = s.strip_prefix("0b") => u32::from_str_radix(bin, 2),
        s => s.parse(),
    };

    parsed.map_err(|_| anyhow!("Invalid immediate: {}", imm))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cond() {
        assert_eq!(parse_cond("eq").unwrap(), 0b001);
        assert_eq!(parse_cond("ne").unwrap(), 0b010);
        assert_eq!(parse_cond("lt").unwrap(), 0b011);
        assert_eq!(parse_cond("le").unwrap(), 0b100);
        assert_eq!(parse_cond("gt").unwrap(), 0b101);
        assert_eq!(parse_cond("ge").unwrap(), 0b110);
        assert!(parse_cond("invalid").is_err()); // TODO: 'thiserror'
    }

    #[test]
    fn test_parse_reg() {
        assert_eq!(parse_reg("r0").unwrap(), 0);
        assert_eq!(parse_reg("s3").unwrap(), 9);
        assert_eq!(parse_reg("a5").unwrap(), 23);
        assert!(parse_reg("invalid").is_err());
    }

    #[test]
    fn test_parse_imm() {
        assert_eq!(parse_imm("42").unwrap(), 42);
        assert_eq!(parse_imm("0x2A").unwrap(), 42);
        assert_eq!(parse_imm("0b101010").unwrap(), 42);
        assert!(parse_imm("r1").is_err());
        assert!(parse_imm("invalid").is_err());
    }

    #[test]
    fn test_encode_r() {
        let instr = INSTRUCTIONS.get("add").unwrap();

        assert!(instr.encode(None, &["r1", "r2"]).is_err());
        assert!(instr.encode(None, &["r1", "r2", "r3", "r4"]).is_err());
        assert!(instr.encode(None, &["r1", "r2", "rrr"]).is_err());
        assert!(instr.encode(None, &["r1", "r2", "123"]).is_err());
        assert!(instr.encode(Some("invalid"), &["r1", "r2", "r3"]).is_err());

        let encoded = instr.encode(Some("lt"), &["r1", "r2", "r3"]).unwrap();
        assert_eq!(encoded, 0b_0000_000_011_00001_00010_0000000_00011);
    }

    #[test]
    fn test_encode_i() {
        let instr = INSTRUCTIONS.get("addi").unwrap();

        assert!(instr.encode(None, &["r1", "r2"]).is_err());
        assert!(instr.encode(None, &["r1", "r2", "r3", "r4"]).is_err());
        assert!(instr.encode(None, &["r1", "rrr", "123"]).is_err());
        assert!(instr.encode(None, &["r1", "r2", "r3"]).is_err());
        assert!(instr.encode(Some("invalid"), &["r1", "r2", "123"]).is_err());

        let encoded = instr.encode(Some("ge"), &["r4", "r5", "0b100"]).unwrap();
        assert_eq!(encoded, 0b_0100_000_110_00100_00101_000000000100);
    }

    #[test]
    fn test_enocde_b() {
        let instr = INSTRUCTIONS.get("beq").unwrap();

        let encoded = instr.encode(Some("ne"), &["r1", "r2", "3456"]).unwrap();
        // 3456 = 0b_11011_0000000
        assert_eq!(encoded, 0b_1001_001_010_11011_00001_0000000_00010);
    }

    #[test]
    fn test_encode_u() {
        let instr = INSTRUCTIONS.get("lui").unwrap();

        assert!(instr.encode(None, &["r1"]).is_err());
        assert!(instr.encode(None, &["r1", "r2"]).is_err());
        assert!(instr.encode(None, &["r1", "r2", "r3"]).is_err());
        assert!(instr.encode(Some("eq"), &["r3", "0xABCDE"]).is_err());

        let encoded = instr.encode(None, &["r3", "0xABCDE"]).unwrap();
        // 0xABCDE = 0b_101_01011110011011110
        assert_eq!(encoded, 0b_1000_011_101_00011_01011110011011110);
    }
}
