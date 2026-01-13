pub use insta::assert_snapshot;

use crate::{
    assembler::Context, instructions::*, macro_instructions::*, operand::OperandValue,
    utils::fmt_line,
};

pub fn instr(cmd: &str) -> impl Fn(&str, &[&str]) -> String {
    let instr = INSTRUCTIONS.get(cmd).unwrap();
    move |cond, ops| match instr.encode(
        &Context::test(),
        0,
        if cond.is_empty() { None } else { Some(cond) },
        &ops.iter()
            .map(|e| OperandValue::from(*e))
            .collect::<Vec<_>>(),
    ) {
        Ok(code) => fmt_bits(code),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn mc_instr(cmd: &str) -> impl Fn(&str, &[&str]) -> String {
    let ps_instr = MACRO_INSTRUCTIONS.get(cmd).unwrap();
    |cond, ops| match ps_instr.expand(
        &Context::test(),
        0,
        cmd,
        if cond.is_empty() { None } else { Some(cond) },
        &ops.iter()
            .map(|e| OperandValue::from(*e))
            .collect::<Vec<_>>(),
    ) {
        Ok(expanded) => match expanded {
            Some(expanded) => expanded
                .into_iter()
                .map(|(name, cond, ops)| fmt_line(name, cond, ops))
                .collect::<Vec<_>>()
                .join("; "),
            None => "".to_string(),
        },
        Err(e) => format!("Error: {}", e),
    }
}

fn fmt_bits(n: u32) -> String {
    const GROUP: [usize; 7] = [4, 3, 3, 5, 5, 7, 5]; // TODO: format according to instruction type
    let bits = format!("{:032b}", n);

    GROUP
        .iter()
        .scan(0, |i, &n| {
            let s = &bits[*i..*i + n];
            *i += n;
            Some(s)
        })
        .collect::<Vec<_>>()
        .join(" ")
}
