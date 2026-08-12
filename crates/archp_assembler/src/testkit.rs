use smallvec::SmallVec;

use crate::{
    assembler::Context, instructions::*, macro_instructions::*, parser::operand::operand,
    pseudo_instructions::PSEUDO_INSTRUCTIONS, utils::fmt::fmt_line,
};

pub fn instr(cmd: &str) -> impl Fn(&[&str]) -> String {
    let instr = INSTRUCTIONS.get(cmd).unwrap();
    move |ops| {
        let mut v = SmallVec::new();
        ops.iter().for_each(|op| {
            operand(&Context::default(), op, &mut v).unwrap();
        });
        match instr.encode(&Context::test(), 0, &v) {
            Ok(code) => fmt_bits(code),
            Err(e) => format!("Error: {}", e),
        }
    }
}

pub fn mc_instr(cmd: &str) -> impl Fn(&[&str]) -> String {
    let ps_instr = MACRO_INSTRUCTIONS.get(cmd).unwrap();
    |ops| {
        let mut v = SmallVec::new();
        ops.iter().for_each(|op| {
            operand(&Context::default(), op, &mut v).unwrap();
        });
        match ps_instr.expand(&Context::test(), 0, cmd, &v) {
            Ok(expanded) => match expanded {
                Some(expanded) => expanded
                    .into_iter()
                    .map(|(name, ops)| fmt_line(name, &ops))
                    .collect::<Vec<_>>()
                    .join("; "),
                None => "".to_string(),
            },
            Err(e) => format!("Error: {}", e),
        }
    }
}

pub fn ps_instr(cmd: &str) -> impl Fn(&[&str]) -> String {
    let ps_instr = PSEUDO_INSTRUCTIONS.get(cmd).unwrap();
    |ops| {
        let mut v = SmallVec::new();
        ops.iter().for_each(|op| {
            operand(&Context::default(), op, &mut v).unwrap();
        });
        match ps_instr.expand(&Context::test(), &v) {
            Ok(expanded) => expanded
                .into_iter()
                .map(|(name, ops)| fmt_line(name, &ops))
                .collect::<Vec<_>>()
                .join("; "),
            Err(e) => format!("Error: {}", e),
        }
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
