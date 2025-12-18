use anyhow::{Result, bail};

pub fn parse_cond(cond: &str) -> Result<u32> {
    match cond {
        "eq" => Ok(0b001),
        "ne" => Ok(0b010),
        "lt" => Ok(0b011),
        "ge" => Ok(0b100),
        "gt" => Ok(0b101),
        "le" => Ok(0b110),
        _ => bail!("Invalid condition: {}", cond),
    }
}

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn parse_cond() {
        let f = |s| match super::parse_cond(s) {
            Ok(n) => format!("{n}"),
            Err(e) => format!("Error: {e}"),
        };
        assert_snapshot!(f("eq"), @"1");
        assert_snapshot!(f("ne"), @"2");
        assert_snapshot!(f("lt"), @"3");
        assert_snapshot!(f("ge"), @"4");
        assert_snapshot!(f("gt"), @"5");
        assert_snapshot!(f("le"), @"6");
        assert_snapshot!(f("invalid"), @"Error: Invalid condition: invalid");
    }
}
