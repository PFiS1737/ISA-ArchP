use nom::Parser;
use smallvec::SmallVec;

use crate::{
    context::Context,
    operand::Operand,
    parser::{Result, expression::expr, ident, parens},
};

pub fn operand<'ctx, 'src: 'ctx>(
    ctx: &'ctx Context<'src>,
    input: &'src str,
    out: &mut SmallVec<[Operand<'src>; 3]>,
) -> Result<'src, ()> {
    // case 1: (ident)
    if let Ok((input, ident)) = parens(ident).parse(input) {
        out.push(Operand::Ident(ident));
        out.push(Operand::Num(0));
        return Ok((input, ()));
    }

    // case 2: expr
    let (input, expr) = expr(input)?;
    out.push(expr.eval_to_operand_with(&ctx.equates)?);

    // case 3: expr(ident)
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
