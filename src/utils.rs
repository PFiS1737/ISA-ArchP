use std::iter::repeat_n;

use crate::operand::OperandValue;

pub fn align_tabbed_lines(lines: &[String]) -> impl Iterator<Item = String> {
    let split_lines: Vec<Vec<&str>> = lines
        .iter()
        .map(|line| line.split('\t').collect())
        .collect();

    let cols = split_lines[0].len();

    let mut max_width = vec![0usize; cols];
    for row in &split_lines {
        for (i, part) in row.iter().enumerate() {
            max_width[i] = max_width[i].max(part.len());
        }
    }

    split_lines.into_iter().map(move |row| {
        let mut out = String::new();

        for (i, part) in row.iter().enumerate() {
            out += part;

            if i + 1 < cols {
                let padding = max_width[i] - part.len() + 2;
                out.extend(repeat_n(' ', padding));
            }
        }

        out.trim_end().to_string()
    })
}

pub fn fmt_line(name: &str, cond: Option<&str>, ops: Vec<OperandValue>) -> String {
    let ops = ops
        .into_iter()
        .map(|e| match e {
            OperandValue::StringSlice(s) => s.to_string(),
            OperandValue::Unsigned(n) => fmt_hex(n),
        })
        .collect::<Vec<_>>();

    let mut line = String::with_capacity(
        name.len()
            + cond.map(|c| 1 + c.len()).unwrap_or(0)
            + if ops.is_empty() { 0 } else { 1 }
            + ops.iter().map(|o| o.len()).sum::<usize>()
            + ops.len().saturating_sub(1),
    );

    line.push_str(name);

    if let Some(c) = cond {
        line.push('.');
        line.push_str(c);
    }

    if !ops.is_empty() {
        for op in ops {
            line.push(' ');
            line.push_str(&op);
        }
    }

    line
}

pub fn fmt_hex(n: u32) -> String {
    if n < 256 {
        n.to_string()
    } else {
        format!("0x{:X}", n)
    }
}
