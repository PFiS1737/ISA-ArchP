use nom::{Parser, character::complete::char, sequence::delimited};
use smallvec::SmallVec;

use crate::{
    assembler::Context,
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

pub fn operand<'ctx, 'src: 'ctx>(
    ctx: &'ctx Context<'src>,
    input: &'src str,
    out: &mut SmallVec<[Operand<'src>; 3]>,
) -> Result<'src, ()> {
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
    out.push(expr.eval_to_operand_with(&ctx.equates)?);

    // case 4: expr(ident)
    let Ok((input, ident)) = parens(ident).parse(input) else {
        return Ok((input, ()));
    };

    out.insert(out.len() - 1, Operand::Ident(ident));

    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    fn test(input: &str) -> String {
        let mut out = SmallVec::new();
        match operand(&Context::default(), input, &mut out) {
            Ok((rest, _)) => format!("unparsed input: {:?}\n{:#?}", rest, out),
            Err(e) => format!("Error: {}", e),
        }
    }

    #[test]
    fn operand_string() {
        assert_snapshot!(test(r#""hello""#), @r#"
        unparsed input: ""
        [
            String(
                "hello",
            ),
        ]
        "#);
        assert_snapshot!(test(r#""hello \"world\" !""#), @r#"
        unparsed input: ""
        [
            String(
                "hello \\\"world\\\" !",
            ),
        ]
        "#);
        assert_snapshot!(test(r#""hello \\""#), @r#"
        unparsed input: ""
        [
            String(
                "hello \\\\",
            ),
        ]
        "#);
        assert_snapshot!(test(r#""hello \\\"abc""#), @r#"
        unparsed input: ""
        [
            String(
                "hello \\\\\\\"abc",
            ),
        ]
        "#);
    }

    #[test]
    fn operand_ident_num() {
        assert_snapshot!(test("abc"), @r#"
        unparsed input: ""
        [
            Ident(
                "abc",
            ),
        ]
        "#);
        assert_snapshot!(test("42"), @r#"
        unparsed input: ""
        [
            Num(
                42,
            ),
        ]
        "#);
        assert_snapshot!(test("(abc)"), @r#"
        unparsed input: ""
        [
            Ident(
                "abc",
            ),
            Num(
                0,
            ),
        ]
        "#);
        assert_snapshot!(test("3(abc)"), @r#"
        unparsed input: ""
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

    #[test]
    fn operand_expr() {
        assert_snapshot!(test("1 + 2"), @r#"
        unparsed input: ""
        [
            Num(
                3,
            ),
        ]
        "#);
        assert_snapshot!(test("1 + 2(abc)"), @r#"
        unparsed input: ""
        [
            Ident(
                "abc",
            ),
            Num(
                3,
            ),
        ]
        "#);
        assert_snapshot!(test("5 + 2(abc) + 3"), @r#"
        unparsed input: "+ 3"
        [
            Ident(
                "abc",
            ),
            Num(
                7,
            ),
        ]
        "#);
        assert_snapshot!(test("foo+1"), @r#"
        unparsed input: ""
        [
            Addition(
                "foo",
                1,
            ),
        ]
        "#);
        assert_snapshot!(test("foo-1"), @r#"
        unparsed input: ""
        [
            Addition(
                "foo",
                -1,
            ),
        ]
        "#);
        assert_snapshot!(test("1+foo"), @r#"
        unparsed input: ""
        [
            Addition(
                "foo",
                1,
            ),
        ]
        "#);
        assert_snapshot!(test("1-foo"), @"Error: Parsing Failure: Eval(NotAbsolute)");
        assert_snapshot!(test("(foo+1)+1"), @r#"
        unparsed input: ""
        [
            Addition(
                "foo",
                2,
            ),
        ]
        "#);
        assert_snapshot!(test("(foo+1)-1"), @r#"
        unparsed input: ""
        [
            Ident(
                "foo",
            ),
        ]
        "#);
        assert_snapshot!(test("1+(foo+1)"), @r#"
        unparsed input: ""
        [
            Addition(
                "foo",
                2,
            ),
        ]
        "#);
        assert_snapshot!(test("1-(foo+1)"), @"Error: Parsing Failure: Eval(NotAbsolute)");
    }
}
