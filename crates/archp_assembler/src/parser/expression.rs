use anyhow::anyhow;
use nom::{
    Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{bin_digit1, char, digit1, hex_digit1},
    combinator::{map, map_res, opt},
    sequence::preceded,
};

use crate::parser::{Result, ident, parens, types::expression::*, ws};

fn binary(input: &str) -> Result<'_, i64> {
    map_res(preceded(tag("0b"), bin_digit1), |s: &str| {
        i64::from_str_radix(s, 2)
    })
    .parse(input)
}

fn decimal(input: &str) -> Result<'_, i64> {
    map_res(digit1, |s: &str| s.parse::<i64>()).parse(input)
}

fn hexadecimal(input: &str) -> Result<'_, i64> {
    map_res(preceded(tag("0x"), hex_digit1), |s: &str| {
        i64::from_str_radix(s, 16)
    })
    .parse(input)
}

fn number(input: &str) -> Result<'_, Expr<'_>> {
    map(ws(alt((hexadecimal, binary, decimal))), Expr::Num).parse(input)
}

fn primary(input: &str) -> Result<'_, Expr<'_>> {
    alt((number, map(ident, Expr::Ident), parens(expr))).parse(input)
}

fn unary(input: &str) -> Result<'_, Expr<'_>> {
    let (input, opt_op) = opt(ws(alt((
        map(char('-'), |_| UnaryOp::Neg),
        map(char('~'), |_| UnaryOp::Not),
    ))))
    .parse(input)?;

    if let Some(op) = opt_op {
        let (input, rhs) = unary(input)?;

        Ok((input, try_eval_unary(op, rhs)?))
    } else {
        primary(input)
    }
}

fn precedence(op: BinaryOp) -> u8 {
    // NOTE: See https://sourceware.org/binutils/docs/as/Infix-Ops.html
    //
    // NOTE: See https://git.sr.ht/~sourceware/binutils-gdb/tree/master/item/gas/expr.c
    //       static operator_rankT op_rank[O_max] = { ... }
    match op {
        BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod | BinaryOp::Shl | BinaryOp::Shr => 6,
        BinaryOp::And | BinaryOp::Xor | BinaryOp::Or | BinaryOp::OrNot => 5,
        BinaryOp::Add | BinaryOp::Sub => 4,
        BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Le | BinaryOp::Gt | BinaryOp::Ge => {
            3
        },
        BinaryOp::LogicalAnd => 2,
        BinaryOp::LogicalOr => 1,
    }
}

fn binary_op(input: &str) -> Result<'_, BinaryOp> {
    ws(alt((
        map(tag("<<"), |_| BinaryOp::Shl),
        map(tag(">>"), |_| BinaryOp::Shr),
        map(tag("=="), |_| BinaryOp::Eq),
        map(tag("<>"), |_| BinaryOp::Ne),
        map(tag("!="), |_| BinaryOp::Ne),
        map(tag("<="), |_| BinaryOp::Le),
        map(tag(">="), |_| BinaryOp::Ge),
        map(tag("&&"), |_| BinaryOp::LogicalAnd),
        map(tag("||"), |_| BinaryOp::LogicalOr),
        map(tag("*"), |_| BinaryOp::Mul),
        map(tag("/"), |_| BinaryOp::Div),
        map(tag("%"), |_| BinaryOp::Mod),
        map(tag("&"), |_| BinaryOp::And),
        map(tag("^"), |_| BinaryOp::Xor),
        map(tag("|"), |_| BinaryOp::Or),
        map(tag("!"), |_| BinaryOp::OrNot),
        map(tag("+"), |_| BinaryOp::Add),
        map(tag("-"), |_| BinaryOp::Sub),
        map(tag("<"), |_| BinaryOp::Lt),
        map(tag(">"), |_| BinaryOp::Gt),
    )))
    .parse(input)
}

fn expr_bp(input: &str, min_bp: u8) -> Result<'_, Expr<'_>> {
    let (mut input, mut lhs) = unary(input)?;

    while let Ok((next_input, op)) = binary_op(input) {
        let prec = precedence(op);
        if prec < min_bp {
            break;
        }

        let (next_input, rhs) = expr_bp(next_input, prec + 1)?;

        lhs = try_eval_binary(lhs, op, rhs)?;

        input = next_input;
    }

    Ok((input, lhs))
}

pub fn expr(input: &str) -> Result<'_, Expr<'_>> {
    expr_bp(input, 0)
}

pub fn parse_expr(input: &str) -> anyhow::Result<(&str, Expr<'_>)> {
    expr(input).map_err(|e| anyhow!("Error parsing expression '{}': {}", input, e))
}

#[cfg(test)]
mod tests {
    use insta::assert_snapshot;

    use super::*;

    fn parse_ok(input: &str) -> Expr<'_> {
        let (rest, expr) = parse_expr(input).expect("parse failed");
        assert!(rest.trim().is_empty(), "unparsed input: {:?}", rest);
        expr
    }

    #[test]
    fn test_number() {
        assert_snapshot!(parse_ok("42"), @"42");
    }

    #[test]
    fn test_ident() {
        assert_snapshot!(parse_ok("abc123"), @"abc123");
    }

    #[test]
    fn test_unary() {
        assert_snapshot!(parse_ok("-1"), @"-1");
        assert_snapshot!(parse_ok("~1"), @"-2");
    }

    #[test]
    fn test_binary() {
        assert_snapshot!(parse_ok("1 + 2 * 3 / 2 % 2"), @"2");
        assert_snapshot!(parse_ok("1 + 2 << 3"), @"17");
        assert_snapshot!(parse_ok("1 & 2 ^ 3 | 4 ! 5"), @"-1");
        assert_snapshot!(parse_ok("1 == 2 && 3 != 4 || 5 < 6"), @"1");
        assert_snapshot!(parse_ok("1 - 2 - 3"), @"-4");
        assert_snapshot!(parse_ok("(1 + 2) * 3"), @"9");
    }

    #[test]
    fn test_unary_with_binary() {
        assert_snapshot!(parse_ok("-1 + 2"), @"1");
    }

    #[test]
    fn test_complex_expression() {
        assert_snapshot!(parse_ok("1 + 2 ! 10 * 3 << 1 & 7 ^ 2 | ~4"), @"-4");
    }

    #[test]
    fn test_whitespace() {
        assert_snapshot!(parse_ok(" 1  +   2 *  3 "), @"7");
    }

    #[test]
    fn test_number_hex() {
        assert_snapshot!(parse_ok("0x2A"), @"42");
    }

    #[test]
    fn test_number_binary() {
        assert_snapshot!(parse_ok("0b101010"), @"42");
    }

    #[test]
    fn test_invalid_input() {
        assert!(parse_expr("1 +").is_err());
        assert!(parse_expr("<< 1").is_err());
    }
}
