use anyhow::{Result, anyhow};
use nom::{
    IResult, Parser,
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{bin_digit1, char, digit1, hex_digit1},
    combinator::{map, map_res},
    sequence::{delimited, preceded},
};

use crate::parser::{types::expression::*, ws};

// ---------- parser ----------

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

fn ident(input: &str) -> IResult<&str, Expr<'_>> {
    map(
        ws(take_while1(|c: char| c.is_alphabetic() || c == '_')),
        Expr::Ident,
    )
    .parse(input)
}

fn parens(input: &str) -> IResult<&str, Expr<'_>> {
    delimited(ws(char('(')), expr, ws(char(')'))).parse(input)
}

fn primary(input: &str) -> IResult<&str, Expr<'_>> {
    alt((number, ident, parens)).parse(input)
}

// ---------- unary ----------

fn unary(input: &str) -> IResult<&str, Expr<'_>> {
    let (input, opt_op) =
        nom::combinator::opt(ws(alt((char('+'), char('-'), char('~'))))).parse(input)?;

    if let Some(op) = opt_op {
        let (input, rhs) = unary(input)?;
        let op = match op {
            '+' => UnaryOp::Pos,
            '-' => UnaryOp::Neg,
            '~' => UnaryOp::Not,
            _ => unreachable!(),
        };
        Ok((input, Expr::Unary {
            op,
            rhs: Box::new(rhs),
        }))
    } else {
        primary(input)
    }
}

// ---------- precedence（对齐 C / GAS） ----------

fn precedence(op: BinaryOp) -> u8 {
    match op {
        BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => 6,
        BinaryOp::Add | BinaryOp::Sub => 5,
        BinaryOp::Shl | BinaryOp::Shr => 4,
        BinaryOp::And => 3,
        BinaryOp::Xor => 2,
        BinaryOp::Or => 1,
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

// ---------- Pratt ----------

fn expr_bp(input: &str, min_bp: u8) -> IResult<&str, Expr<'_>> {
    let (mut input, mut lhs) = unary(input)?;

    while let Ok((next_input, op)) = binary_op(input) {
        let prec = precedence(op);
        if prec < min_bp {
            break;
        }

        let (next_input, rhs) = expr_bp(next_input, prec + 1)?;

        lhs = Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        };

        input = next_input;
    }

    Ok((input, lhs))
}

fn expr(input: &str) -> IResult<&str, Expr<'_>> {
    expr_bp(input, 0)
}

pub fn parse_expr(input: &str) -> Result<(&str, Expr<'_>)> {
    expr(input).map_err(|e| anyhow!("Error parsing expression '{}': {}", input, e))
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    fn parse_ok(input: &str) -> Expr<'_> {
        let (rest, expr) = parse_expr(input).expect("parse failed");
        assert!(rest.trim().is_empty(), "unparsed input: {:?}", rest);
        expr
    }

    #[test]
    fn test_number() {
        assert_debug_snapshot!(parse_ok("42"), @"
        Num(
            42,
        )
        ");
    }

    #[test]
    fn test_ident() {
        assert_debug_snapshot!(parse_ok("abc"), @r#"
        Ident(
            "abc",
        )
        "#);
    }

    #[test]
    fn test_unary_all() {
        assert_debug_snapshot!(parse_ok("+1"), @"
        Unary {
            op: Pos,
            rhs: Num(
                1,
            ),
        }
        ");

        assert_debug_snapshot!(parse_ok("-1"), @"
        Unary {
            op: Neg,
            rhs: Num(
                1,
            ),
        }
        ");

        assert_debug_snapshot!(parse_ok("~1"), @"
        Unary {
            op: Not,
            rhs: Num(
                1,
            ),
        }
        ");
    }

    #[test]
    fn test_mul_div_mod_precedence() {
        // 1 + (2 * 3) / 2 % 2
        assert_debug_snapshot!(parse_ok("1 + 2 * 3 / 2 % 2"), @"
        Binary {
            lhs: Num(
                1,
            ),
            op: Add,
            rhs: Binary {
                lhs: Binary {
                    lhs: Binary {
                        lhs: Num(
                            2,
                        ),
                        op: Mul,
                        rhs: Num(
                            3,
                        ),
                    },
                    op: Div,
                    rhs: Num(
                        2,
                    ),
                },
                op: Mod,
                rhs: Num(
                    2,
                ),
            },
        }
        ");
    }

    #[test]
    fn test_shift_precedence() {
        // (1 + 2) << 3
        assert_debug_snapshot!(parse_ok("1 + 2 << 3"), @"
        Binary {
            lhs: Binary {
                lhs: Num(
                    1,
                ),
                op: Add,
                rhs: Num(
                    2,
                ),
            },
            op: Shl,
            rhs: Num(
                3,
            ),
        }
        ");
    }

    #[test]
    fn test_and_xor_or_precedence() {
        // ((1 & 2) ^ 3) | 4
        assert_debug_snapshot!(parse_ok("1 & 2 ^ 3 | 4"), @"
        Binary {
            lhs: Binary {
                lhs: Binary {
                    lhs: Num(
                        1,
                    ),
                    op: And,
                    rhs: Num(
                        2,
                    ),
                },
                op: Xor,
                rhs: Num(
                    3,
                ),
            },
            op: Or,
            rhs: Num(
                4,
            ),
        }
        ");
    }

    #[test]
    fn test_left_associativity() {
        // ((1 - 2) - 3)
        assert_debug_snapshot!(parse_ok("1 - 2 - 3"), @"
        Binary {
            lhs: Binary {
                lhs: Num(
                    1,
                ),
                op: Sub,
                rhs: Num(
                    2,
                ),
            },
            op: Sub,
            rhs: Num(
                3,
            ),
        }
        ");
    }

    #[test]
    fn test_parentheses() {
        assert_debug_snapshot!(parse_ok("(1 + 2) * 3"), @"
        Binary {
            lhs: Binary {
                lhs: Num(
                    1,
                ),
                op: Add,
                rhs: Num(
                    2,
                ),
            },
            op: Mul,
            rhs: Num(
                3,
            ),
        }
        ");
    }

    #[test]
    fn test_unary_with_binary() {
        assert_debug_snapshot!(parse_ok("-1 + 2"), @"
        Binary {
            lhs: Unary {
                op: Neg,
                rhs: Num(
                    1,
                ),
            },
            op: Add,
            rhs: Num(
                2,
            ),
        }
        ");
    }

    #[test]
    fn test_complex_expression() {
        assert_debug_snapshot!(parse_ok("1 + 2 * 3 << 1 & 7 ^ 2 | ~4"), @"
        Binary {
            lhs: Binary {
                lhs: Binary {
                    lhs: Binary {
                        lhs: Binary {
                            lhs: Num(
                                1,
                            ),
                            op: Add,
                            rhs: Binary {
                                lhs: Num(
                                    2,
                                ),
                                op: Mul,
                                rhs: Num(
                                    3,
                                ),
                            },
                        },
                        op: Shl,
                        rhs: Num(
                            1,
                        ),
                    },
                    op: And,
                    rhs: Num(
                        7,
                    ),
                },
                op: Xor,
                rhs: Num(
                    2,
                ),
            },
            op: Or,
            rhs: Unary {
                op: Not,
                rhs: Num(
                    4,
                ),
            },
        }
        ");
    }

    #[test]
    fn test_whitespace() {
        assert_debug_snapshot!(parse_ok(" 1  +   2 *  3 "), @"
        Binary {
            lhs: Num(
                1,
            ),
            op: Add,
            rhs: Binary {
                lhs: Num(
                    2,
                ),
                op: Mul,
                rhs: Num(
                    3,
                ),
            },
        }
        ");
    }

    #[test]
    fn test_number_hex() {
        assert_debug_snapshot!(parse_ok("0x2A"), @"
        Num(
            42,
        )
        ");
    }

    #[test]
    fn test_number_binary() {
        assert_debug_snapshot!(parse_ok("0b101010"), @"
        Num(
            42,
        )
        ");
    }

    #[test]
    fn test_invalid_input() {
        assert!(parse_expr("1 +").is_err());
        assert!(parse_expr("<< 1").is_err());
    }
}
