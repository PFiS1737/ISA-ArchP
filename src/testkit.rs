pub use crate::instructions::*;
pub use crate::pseudo_instructions::*;
pub use insta::assert_snapshot;

pub fn instr(cmd: &str) -> impl Fn(&str, &[&str]) -> String {
    let instr = INSTRUCTIONS.get(cmd).unwrap();
    |cond, ops| match instr.encode(if cond.is_empty() { None } else { Some(cond) }, ops) {
        Ok(code) => fmt_bits(code),
        Err(e) => format!("Error: {}", e),
    }
}

pub fn ps_instr(cmd: &str) -> impl Fn(&[&str]) -> String {
    let ps_instr = PSEUDO_INSTRUCTIONS.get(cmd).unwrap();
    |ops| match ps_instr.expand(ops) {
        Ok(expanded) => expanded
            .into_iter()
            .map(|(name, ops)| format!("{name} {}", ops.join(" ")))
            .collect::<Vec<_>>()
            .join("; "),
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
