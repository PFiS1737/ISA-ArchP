use crate::operand::Operand;

pub fn fmt_hex(n: impl FormatHex) -> String {
    n.fmt_hex()
}

pub fn fmt_line(name: &str, ops: &[Operand]) -> String {
    let ops = ops
        .iter()
        // TODO: remove this
        .map(|e| match e {
            Operand::Num(n) => fmt_hex(*n),
            Operand::Ident(s) => s.to_string(),
            Operand::Addition(s, n) => format!("{}{:+}", s, n),
        })
        .collect::<Vec<_>>();

    let mut line = String::with_capacity(
        name.len()
            + if ops.is_empty() { 0 } else { 1 }
            + ops.iter().map(|o| o.len()).sum::<usize>()
            + ops.len().saturating_sub(1),
    );

    line.push_str(name);

    if !ops.is_empty() {
        for op in ops {
            line.push(' ');
            line.push_str(&op);
        }
    }

    line
}

pub trait FormatHex {
    fn fmt_hex(&self) -> String;
}

macro impl_format_hex {
    ($($t:ty),*) => {
        $(
            impl FormatHex for $t {
                fn fmt_hex(&self) -> String {
                    let str = self.to_string();
                    let hex = format!("{:#X}", self);

                    if self >= &0 {
                        if str.len() > 3 { hex } else { str }
                    } else {
                        if str.len() > 5 { hex } else { str }
                    }
                }
            }
        )*
    },
}

impl_format_hex!(u32, i32, i64);
