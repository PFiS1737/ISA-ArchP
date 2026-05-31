use crate::operand::OperandValue;

pub fn fmt_hex(n: impl FormatHex) -> String {
    n.fmt_hex()
}

pub fn fmt_line(name: &str, ops: Vec<OperandValue>) -> String {
    let ops = ops
        .into_iter()
        .map(|e| match e {
            OperandValue::StringSlice(s) => s.to_string(),
            OperandValue::Unsigned(n) => fmt_hex(n), // FIXME: 根据指令显示不同位数
            OperandValue::Signed(n) => fmt_hex(n),
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
            format!("0x{:X}", self)
        }
    }
}

// FIXME: maybe better

impl FormatHex for i32 {
    fn fmt_hex(&self) -> String {
        if *self >= -256 && *self < 256 {
            self.to_string()
        } else {
            format!("0x{:X}", *self)
        }
    }
}

impl FormatHex for u64 {
    fn fmt_hex(&self) -> String {
        if self < &256 {
            self.to_string()
        } else {
            format!("0x{:X}", self)
        }
    }
}

impl FormatHex for i64 {
    fn fmt_hex(&self) -> String {
        if *self >= -256 && *self < 256 {
            self.to_string()
        } else {
            format!("0x{:X}", *self)
        }
    }
}
