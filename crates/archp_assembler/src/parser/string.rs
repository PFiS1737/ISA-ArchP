use nom::{Parser, character::complete::char, sequence::delimited};

use crate::parser::{Error, Result};

fn take_until_unescaped_quote(input: &str) -> Result<'_, &str> {
    let bytes = input.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if bytes[i] == b'"' {
            let mut slash_count = 0;
            let mut j = i;

            while j > 0 && bytes[j - 1] == b'\\' {
                slash_count += 1;
                j -= 1;
            }

            if slash_count % 2 == 0 {
                return Ok((&input[i..], &input[..i]));
            }
        }

        i += 1;
    }

    Err(nom::Err::Error(Error::Nom(nom::error::Error::new(
        input,
        nom::error::ErrorKind::TakeUntil,
    ))))
}

pub fn string(input: &str) -> Result<'_, &str> {
    delimited(char('"'), take_until_unescaped_quote, char('"')).parse(input)
}

#[cfg(test)]
mod tests {

    use insta::assert_snapshot;

    use super::*;

    fn test(input: &str) -> String {
        match string(input) {
            Ok((rest, s)) => format!("unparsed input: {:?}\n{:?}", rest, s),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[test]
    fn operand_string() {
        assert_snapshot!(test(r#""hello""#), @r#"
        unparsed input: ""
        "hello"
        "#);
        assert_snapshot!(test(r#""hello \"world\" !""#), @r#"
        unparsed input: ""
        "hello \\\"world\\\" !"
        "#);
        assert_snapshot!(test(r#""hello \\""#), @r#"
        unparsed input: ""
        "hello \\\\"
        "#);
        assert_snapshot!(test(r#""hello \\\"abc""#), @r#"
        unparsed input: ""
        "hello \\\\\\\"abc"
        "#);
    }
}
