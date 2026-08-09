use crate::operand::Operand;

pub fn fmt_hex(n: impl FormatHex) -> String {
    n.fmt_hex()
}

pub fn fmt_line(name: &str, ops: &[Operand]) -> String {
    let ops = ops
        .iter()
        .map(|e| match e {
            Operand::Num(n) => fmt_hex(*n), // FIXME: 根据指令显示不同位数
            Operand::Ident(s) => s.to_string(),

            // FIXME: workaround for something like '-1'
            Operand::Expr(e) => e.eval_with(&|_| None).unwrap().to_string(),
            // TODO: impl
            _ => unimplemented!("fmt_line: {}", e),
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

impl FormatHex for u32 {
    fn fmt_hex(&self) -> String {
        if self < &256 {
            self.to_string()
        } else {
            format!("{:#X}", self)
        }
    }
}

// FIXME: maybe better

impl FormatHex for i32 {
    fn fmt_hex(&self) -> String {
        if *self >= -256 && *self < 256 {
            self.to_string()
        } else {
            format!("{:#X}", *self)
        }
    }
}

impl FormatHex for u64 {
    fn fmt_hex(&self) -> String {
        if self < &256 {
            self.to_string()
        } else {
            format!("{:#X}", self)
        }
    }
}

impl FormatHex for i64 {
    fn fmt_hex(&self) -> String {
        if *self >= -256 && *self < 256 {
            self.to_string()
        } else {
            format!("{:#X}", *self)
        }
    }
}
