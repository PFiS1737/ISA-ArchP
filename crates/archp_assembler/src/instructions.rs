#[path = "_generated/instructions.rs"]
mod generated;

use anyhow::Result;
pub use generated::get_by_name;

use crate::{
    codec::{instruction::encode_instruction, operands::encode_operands},
    context::Context,
    operand::Operand,
    types::{InstructionType, OperandType},
};

pub struct Instruction {
    pub name: &'static str,
    pub opcode: u32,
    pub funct3: u32,
    pub itype: InstructionType,
    pub format: &'static [OperandType],
}

impl Instruction {
    pub fn encode<'src>(
        &'static self,
        ctx: &mut Context<'src>,
        operands: &[Operand<'src>],
    ) -> Result<u32> {
        let ops = encode_operands(ctx, self.name, self.format, operands)?;
        let code = encode_instruction(self.itype, self.opcode, self.funct3, &ops);
        Ok(code)
    }
}

#[cfg(test)]
macro instr( @($ctx:expr) $name:ident $($ops:expr),* $(;)? ) {{
    $name.encode($ctx, &$crate::operand::ops![$($ops),*])
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
        use generated::ADD;

        assert_snapshot!(test_instr!(ADD "r1", "r2"), @"Error: Instruction 'add' requires 3 operands, got 2");
        assert_snapshot!(test_instr!(ADD "r1", "r2", "r3", "r4"), @"Error: Instruction 'add' requires 3 operands, got 4");
        assert_snapshot!(test_instr!(ADD "r1", "r2", "rrr"), @"Error: Invalid register: rrr");
        assert_snapshot!(test_instr!(ADD "r1", "r2", 123), @"Error: Expected register, got: 123");

        assert_snapshot!(test_instr!(ADD "r1", "r2", "r3"), @"0x00022003");
    }

    #[test]
    fn encode_i() {
        use generated::ADDI;

        assert_snapshot!(test_instr!(ADDI "r1", "r2"), @"Error: Instruction 'addi' requires 3 operands, got 2");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", "r3", "r4"), @"Error: Instruction 'addi' requires 3 operands, got 4");
        assert_snapshot!(test_instr!(ADDI "r1", "rrr", 123), @"Error: Invalid register: rrr");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", "r3"), @"Error: Expected immediate, got: r3");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", 0xFFF), @"Error: Immediate '4095' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", 0x7FF), @"0x010227FF");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", 0xFFFF), @"Error: Immediate '65535' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", 0xFFFFFFFF_i64), @"0x01022FFF");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", -1), @"0x01022FFF");

        assert_snapshot!(test_instr!(ADDI "r1", "r2", 3), @"0x01022003");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", 2047), @"0x010227FF");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", 2048), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", -3), @"0x01022FFD");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", -2048), @"0x01022800");
        assert_snapshot!(test_instr!(ADDI "r1", "r2", -2049), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");

        use generated::SRLI;

        assert_snapshot!(test_instr!(SRLI "r1", "r2", 32), @"Error: Immediate '32' out of range for u5 (0 ..= 31)");
        assert_snapshot!(test_instr!(SRLI "r1", "r2", 31), @"0x0A42201F");
    }

    #[test]
    fn enocde_b() {
        use generated::SW;

        assert_snapshot!(test_instr!(SW "r1", "r2", 3), @"0x11402061");
        assert_snapshot!(test_instr!(SW "r1", "r2", 2047), @"0x115E2FE1");
        assert_snapshot!(test_instr!(SW "r1", "r2", 2048), @"Error: Immediate '2048' out of range for i12 (-2048 ..= 2047)");
        assert_snapshot!(test_instr!(SW "r1", "r2", -3), @"0x117E2FA1");
        assert_snapshot!(test_instr!(SW "r1", "r2", -2048), @"0x11602001");
        assert_snapshot!(test_instr!(SW "r1", "r2", -2049), @"Error: Immediate '-2049' out of range for i12 (-2048 ..= 2047)");
    }

    #[test]
    fn encode_u() {
        use generated::LUI;

        assert_snapshot!(test_instr!(LUI "r1"), @"Error: Instruction 'lui' requires 2 operands, got 1");
        assert_snapshot!(test_instr!(LUI "r1", "r2", "r3"), @"Error: Instruction 'lui' requires 2 operands, got 3");
        assert_snapshot!(test_instr!(LUI "r1", "r2"), @"Error: Expected immediate, got: r2");
        assert_snapshot!(test_instr!(LUI "r3", 0x200000), @"Error: Immediate '2097152' out of range for u20 (0 ..= 1048575)");
        assert_snapshot!(test_instr!(LUI "r3", -123), @"Error: Immediate '-123' out of range for u20 (0 ..= 1048575)");

        assert_snapshot!(test_instr!(LUI "r3", 0xABCDE), @"0x1746BCDE");
    }
}
