pub fn fmt_hex(n: impl FormatHex) -> String {
    n.fmt_hex()
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
