use nom::{Parser, character::complete::char, sequence::delimited};
use smallvec::SmallVec;

use crate::{
    operand::Operand,
    parser::{Error, Result, expression::expr, ident, parens},
};

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

fn string_literal(input: &str) -> Result<'_, &str> {
    delimited(char('"'), take_until_unescaped_quote, char('"')).parse(input)
}

pub fn operand<'a>(input: &'a str, out: &mut SmallVec<[Operand<'a>; 3]>) -> Result<'a, ()> {
    // case 1: "string"
    if let Ok((rest, s)) = string_literal(input) {
        out.push(Operand::String(s));
        return Ok((rest, ()));
    }

    // case 2: (ident)
    if let Ok((input, ident)) = parens(ident).parse(input) {
        out.push(Operand::Ident(ident));
        out.push(Operand::Num(0));
        return Ok((input, ()));
    }

    // case 3: expr
    let (input, expr) = expr(input)?;
    out.push(expr.into());

    // case 4: expr(ident)
    let Ok((input, ident)) = parens(ident).parse(input) else {
        return Ok((input, ()));
    };

    out.insert(out.len() - 1, Operand::Ident(ident));

    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    fn parse_ok(input: &str) -> SmallVec<[Operand<'_>; 3]> {
        let mut out = SmallVec::new();
        let (rest, _) = operand(input, &mut out).expect("parse failed");
        assert!(rest.trim().is_empty(), "unparsed input: {:?}", rest);
        out
    }

    #[test]
    fn test_operand() {
        assert_debug_snapshot!(parse_ok(r#""hello""#), @r#"
        [
            String(
                "hello",
            ),
        ]
        "#);
        assert_debug_snapshot!(parse_ok(r#""hello \"world\" !""#), @r#"
        [
            String(
                "hello \\\"world\\\" !",
            ),
        ]
        "#);
        assert_debug_snapshot!(parse_ok(r#""hello \\""#), @r#"
        [
            String(
                "hello \\\\",
            ),
        ]
        "#);
        assert_debug_snapshot!(parse_ok(r#""hello \\\"abc""#), @r#"
        [
            String(
                "hello \\\\\\\"abc",
            ),
        ]
        "#);
        assert_debug_snapshot!(parse_ok("abc"), @r#"
        [
            Ident(
                "abc",
            ),
        ]
        "#);
        assert_debug_snapshot!(parse_ok("42"), @"
        [
            Num(
                42,
            ),
        ]
        ");
        assert_debug_snapshot!(parse_ok("(abc)"), @r#"
        [
            Ident(
                "abc",
            ),
            Num(
                0,
            ),
        ]
        "#);
        assert_debug_snapshot!(parse_ok("3(abc)"), @r#"
        [
            Ident(
                "abc",
            ),
            Num(
                3,
            ),
        ]
        "#);
    }
}
