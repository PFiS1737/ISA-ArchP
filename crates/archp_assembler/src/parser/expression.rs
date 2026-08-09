use anyhow::{Result, anyhow};
use nom::{
    Err, IResult, Parser,
    branch::alt,
    bytes::complete::tag,
    character::complete::{bin_digit1, char, digit1, hex_digit1},
    combinator::{map, map_res, opt},
    error::{Error, ErrorKind, FromExternalError},
    sequence::preceded,
};

use crate::parser::{ident, parens, types::expression::*, ws};

fn number(input: &str) -> IResult<&str, Expr<'_>> {
    fn dec(input: &str) -> IResult<&str, i64> {
        map_res(digit1, |s: &str| s.parse::<i64>()).parse(input)
    }

    fn hex(input: &str) -> IResult<&str, i64> {
        map_res(preceded(tag("0x"), hex_digit1), |s: &str| {
            i64::from_str_radix(s, 16)
        })
        .parse(input)
    }

    fn binary(input: &str) -> IResult<&str, i64> {
        map_res(preceded(tag("0b"), bin_digit1), |s: &str| {
            i64::from_str_radix(s, 2)
        })
        .parse(input)
    }

    map(ws(alt((hex, binary, dec))), Expr::Num).parse(input)
}

fn primary(input: &str) -> IResult<&str, Expr<'_>> {
    alt((number, map(ident, Expr::Ident), parens(expr))).parse(input)
}

fn unary(input: &str) -> IResult<&str, Expr<'_>> {
    let (input, opt_op) = opt(ws(alt((char('+'), char('-'), char('~'))))).parse(input)?;

    if let Some(op) = opt_op {
        let (input, rhs) = unary(input)?;

        let op = match op {
            '+' => UnaryOp::Pos,
            '-' => UnaryOp::Neg,
            '~' => UnaryOp::Not,
            _ => unreachable!(),
        };

        if let Expr::Num(n) = rhs {
            Ok((
                input,
                Expr::Num(eval_unary(op, n).map_err(|e| {
                    // TODO: map EvalError
                    Err::Failure(Error::from_external_error(input, ErrorKind::Fail, e))
                })?),
            ))
        } else {
            Ok((input, Expr::Unary {
                op,
                rhs: Box::new(rhs),
            }))
        }
    } else {
        primary(input)
    }
}

fn precedence(op: BinaryOp) -> u8 {
    // NOTE: See https://git.sr.ht/~sourceware/binutils-gdb/tree/master/item/gas/expr.c
    //       static operator_rankT op_rank[O_max] = { ... }
    match op {
        BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod | BinaryOp::Shl | BinaryOp::Shr => 3,
        BinaryOp::And | BinaryOp::Xor | BinaryOp::Or => 2,
        BinaryOp::Add | BinaryOp::Sub => 1,
    }
}

fn binary_op(input: &str) -> IResult<&str, BinaryOp> {
    ws(alt((
        map(tag("<<"), |_| BinaryOp::Shl),
        map(tag(">>"), |_| BinaryOp::Shr),
        map(tag("*"), |_| BinaryOp::Mul),
        map(tag("/"), |_| BinaryOp::Div),
        map(tag("%"), |_| BinaryOp::Mod),
        map(tag("+"), |_| BinaryOp::Add),
        map(tag("-"), |_| BinaryOp::Sub),
        map(tag("&"), |_| BinaryOp::And),
        map(tag("^"), |_| BinaryOp::Xor),
        map(tag("|"), |_| BinaryOp::Or),
    )))
    .parse(input)
}

fn expr_bp(input: &str, min_bp: u8) -> IResult<&str, Expr<'_>> {
    let (mut input, mut lhs) = unary(input)?;

    while let Ok((next_input, op)) = binary_op(input) {
        let prec = precedence(op);
        if prec < min_bp {
            break;
        }

        let (next_input, rhs) = expr_bp(next_input, prec + 1)?;

        if let Expr::Num(l) = lhs
            && let Expr::Num(r) = rhs
        {
            lhs = Expr::Num(eval_binary(op, l, r).map_err(|e| {
                // TODO: map EvalError
                Err::Failure(Error::from_external_error(input, ErrorKind::Fail, e))
            })?);
        } else {
            lhs = Expr::Binary {
                lhs: Box::new(lhs),
                op,
                rhs: Box::new(rhs),
            };
        }

        input = next_input;
    }

    Ok((input, lhs))
}

pub fn expr(input: &str) -> IResult<&str, Expr<'_>> {
    expr_bp(input, 0)
}

pub fn parse_expr(input: &str) -> Result<(&str, Expr<'_>)> {
    expr(input).map_err(|e| anyhow!("Error parsing expression '{}': {}", input, e))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testkit::assert_snapshot;

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
    fn test_unary_all() {
        assert_snapshot!(parse_ok("+1"), @"1");
        assert_snapshot!(parse_ok("-1"), @"-1");
        assert_snapshot!(parse_ok("~1"), @"-2");
    }

    #[test]
    fn test_mul_div_mod_precedence() {
        assert_snapshot!(parse_ok("1 + 2 * 3 / 2 % 2"), @"2");
    }

    #[test]
    fn test_shift_precedence() {
        assert_snapshot!(parse_ok("1 + 2 << 3"), @"17");
    }

    #[test]
    fn test_and_xor_or_precedence() {
        assert_snapshot!(parse_ok("1 & 2 ^ 3 | 4"), @"7");
    }

    #[test]
    fn test_left_associativity() {
        assert_snapshot!(parse_ok("1 - 2 - 3"), @"-4");
    }

    #[test]
    fn test_parentheses() {
        assert_snapshot!(parse_ok("(1 + 2) * 3"), @"9");
    }

    #[test]
    fn test_unary_with_binary() {
        assert_snapshot!(parse_ok("-1 + 2"), @"1");
    }

    #[test]
    fn test_complex_expression() {
        assert_snapshot!(parse_ok("1 + 2 * 3 << 1 & 7 ^ 2 | ~4"), @"0");
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
