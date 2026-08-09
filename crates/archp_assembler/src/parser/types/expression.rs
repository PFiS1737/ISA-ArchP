use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub enum Expr<'src> {
    Num(i64),
    Ident(&'src str),
    Unary {
        op: UnaryOp,
        rhs: Box<Expr<'src>>,
    },
    Binary {
        lhs: Box<Expr<'src>>,
        op: BinaryOp,
        rhs: Box<Expr<'src>>,
    },
}

impl Display for Expr<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Num(n) => write!(f, "{}", n),
            Expr::Ident(s) => write!(f, "{}", s),
            Expr::Unary { op, rhs } => write!(f, "{}{}", op, rhs),
            Expr::Binary { lhs, op, rhs } => write!(f, "({} {} {})", lhs, op, rhs),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UnaryOp {
    Pos, // +
    Neg, // -
    Not, // ~
}

impl Display for UnaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            UnaryOp::Pos => "+",
            UnaryOp::Neg => "-",
            UnaryOp::Not => "~",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinaryOp {
    Mul, // *
    Div, // /
    Mod, // %

    Add, // +
    Sub, // -

    Shl, // <<
    Shr, // >>

    And, // &
    Xor, // ^
    Or,  // |
}

impl Display for BinaryOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            BinaryOp::Mul => "*",
            BinaryOp::Div => "/",
            BinaryOp::Mod => "%",

            BinaryOp::Add => "+",
            BinaryOp::Sub => "-",

            BinaryOp::Shl => "<<",
            BinaryOp::Shr => ">>",

            BinaryOp::And => "&",
            BinaryOp::Xor => "^",
            BinaryOp::Or => "|",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum EvalError<'src> {
    #[error("unknown identifier: {}", .0)]
    UnknownIdent(&'src str),

    #[error("shift amount out of range: {}", .0)]
    ShiftOutOfRange(i64),

    #[error("division by zero")]
    DivByZero,
}

pub fn eval_unary<'a>(op: UnaryOp, rhs: i64) -> Result<i64, EvalError<'a>> {
    match op {
        UnaryOp::Pos => Ok(rhs),
        UnaryOp::Neg => Ok(rhs.wrapping_neg()),
        UnaryOp::Not => Ok(!rhs),
    }
}

pub fn eval_binary<'a>(op: BinaryOp, lhs: i64, rhs: i64) -> Result<i64, EvalError<'a>> {
    match op {
        BinaryOp::Mul => Ok(lhs.wrapping_mul(rhs)),
        BinaryOp::Div => {
            if rhs == 0 {
                return Err(EvalError::DivByZero);
            }
            Ok(lhs.wrapping_div(rhs))
        },
        BinaryOp::Mod => {
            if rhs == 0 {
                return Err(EvalError::DivByZero);
            }
            Ok(lhs.wrapping_rem(rhs))
        },

        BinaryOp::Add => Ok(lhs.wrapping_add(rhs)),
        BinaryOp::Sub => Ok(lhs.wrapping_sub(rhs)),

        BinaryOp::Shl => {
            if !(0..32).contains(&rhs) {
                return Err(EvalError::ShiftOutOfRange(rhs));
            }
            Ok(lhs.wrapping_shl(rhs as u32))
        },

        BinaryOp::Shr => {
            if !(0..32).contains(&rhs) {
                return Err(EvalError::ShiftOutOfRange(rhs));
            }
            Ok(lhs.wrapping_shr(rhs as u32))
        },

        BinaryOp::And => Ok(lhs & rhs),
        BinaryOp::Xor => Ok(lhs ^ rhs),
        BinaryOp::Or => Ok(lhs | rhs),
    }
}

impl<'ctx, 'src: 'ctx> Expr<'src> {
    pub fn eval_with<F>(&self, resolve: &F) -> Result<i64, EvalError<'src>>
    where
        F: Fn(&'src str) -> Option<i64>,
    {
        match self {
            Expr::Num(n) => Ok(*n),

            Expr::Ident(name) => resolve(name).ok_or(EvalError::UnknownIdent(name)),

            Expr::Unary { op, rhs } => {
                let r = rhs.eval_with(resolve)?;
                eval_unary(*op, r)
            },

            Expr::Binary { lhs, op, rhs } => {
                let l = lhs.eval_with(resolve)?;
                let r = rhs.eval_with(resolve)?;
                eval_binary(*op, l, r)
            },
        }
    }

    pub fn eval(&self, env: &'ctx HashMap<&'src str, i64>) -> Result<i64, EvalError<'src>> {
        self.eval_with(&|s| env.get(s).copied())
    }

    pub fn partial_eval_with<F>(
        &self,
        resolve: &F,
    ) -> Result<(Expr<'src>, HashSet<&'src str>), EvalError<'src>>
    where
        F: Fn(&'src str) -> Option<i64>,
    {
        match self {
            Expr::Num(_) => Ok((self.clone(), HashSet::new())),

            Expr::Ident(name) => {
                if let Some(num) = resolve(name) {
                    Ok((Expr::Num(num), HashSet::new()))
                } else {
                    let mut set = HashSet::new();
                    set.insert(*name);
                    Ok((self.clone(), set))
                }
            },

            Expr::Unary { op, rhs } => {
                let (r, undef) = rhs.partial_eval_with(resolve)?;

                if let Expr::Num(r) = r {
                    Ok((Expr::Num(eval_unary(*op, r)?), undef))
                } else {
                    Ok((
                        Expr::Unary {
                            op: *op,
                            rhs: Box::new(r),
                        },
                        undef,
                    ))
                }
            },

            Expr::Binary { lhs, op, rhs } => {
                let (l, mut undef_l) = lhs.partial_eval_with(resolve)?;
                let (r, undef_r) = rhs.partial_eval_with(resolve)?;
                undef_l.extend(undef_r);

                if let Expr::Num(l) = l
                    && let Expr::Num(r) = r
                {
                    Ok((Expr::Num(eval_binary(*op, l, r)?), undef_l))
                } else {
                    Ok((
                        Expr::Binary {
                            lhs: Box::new(l),
                            op: *op,
                            rhs: Box::new(r),
                        },
                        undef_l,
                    ))
                }
            },
        }
    }

    pub fn partial_eval(
        &self,
        env: &'ctx HashMap<&'src str, i64>,
    ) -> Result<(Expr<'src>, HashSet<&'src str>), EvalError<'src>> {
        self.partial_eval_with(&|s| env.get(s).copied())
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    fn eval_ok(expr: Expr<'_>) -> i64 {
        expr.eval(&HashMap::new()).unwrap()
    }

    fn bin<'a>(lhs: Expr<'a>, op: BinaryOp, rhs: Expr<'a>) -> Expr<'a> {
        Expr::Binary {
            lhs: Box::new(lhs),
            op,
            rhs: Box::new(rhs),
        }
    }

    fn unary(op: UnaryOp, rhs: Expr<'_>) -> Expr<'_> {
        Expr::Unary {
            op,
            rhs: Box::new(rhs),
        }
    }

    #[test]
    fn test_basic_arith() {
        // 1 + 2 * 3
        let expr = bin(
            Expr::Num(1),
            BinaryOp::Add,
            bin(Expr::Num(2), BinaryOp::Mul, Expr::Num(3)),
        );

        assert_eq!(eval_ok(expr), 1 + 2 * 3);
    }

    #[test]
    fn test_div_mod() {
        assert_eq!(
            eval_ok(bin(Expr::Num(7), BinaryOp::Div, Expr::Num(2))),
            7 / 2
        );
        assert_eq!(
            eval_ok(bin(Expr::Num(7), BinaryOp::Mod, Expr::Num(2))),
            7 % 2
        );
    }

    #[test]
    fn test_xor() {
        assert_eq!(
            eval_ok(bin(Expr::Num(6), BinaryOp::Xor, Expr::Num(3))),
            6 ^ 3
        );
    }

    #[test]
    fn test_shift() {
        assert_eq!(
            eval_ok(bin(Expr::Num(1), BinaryOp::Shl, Expr::Num(3))),
            1 << 3
        );
        assert_eq!(
            eval_ok(bin(Expr::Num(8), BinaryOp::Shr, Expr::Num(2))),
            8 >> 2
        );
    }

    #[test]
    fn test_bitwise() {
        assert_eq!(
            eval_ok(bin(Expr::Num(6), BinaryOp::And, Expr::Num(3))),
            6 & 3
        );
        assert_eq!(
            eval_ok(bin(Expr::Num(4), BinaryOp::Or, Expr::Num(1))),
            4 | 1
        );
    }

    #[test]
    fn test_unary_all() {
        assert_eq!(eval_ok(unary(UnaryOp::Pos, Expr::Num(5))), 5);
        assert_eq!(eval_ok(unary(UnaryOp::Neg, Expr::Num(5))), -5);
        assert_eq!(eval_ok(unary(UnaryOp::Not, Expr::Num(0))), !0);
    }

    #[test]
    fn test_combined() {
        // (1 + 2 * 3) << 1 & 7 ^ 2 | ~4
        let expr = bin(
            bin(
                bin(
                    bin(
                        Expr::Num(1),
                        BinaryOp::Add,
                        bin(Expr::Num(2), BinaryOp::Mul, Expr::Num(3)),
                    ),
                    BinaryOp::Shl,
                    Expr::Num(1),
                ),
                BinaryOp::And,
                Expr::Num(7),
            ),
            BinaryOp::Xor,
            Expr::Num(2),
        );

        let expr = bin(expr, BinaryOp::Or, unary(UnaryOp::Not, Expr::Num(4)));

        assert_eq!(eval_ok(expr), (1 + 2 * 3) << 1 & 7 ^ 2 | !4);
    }

    #[test]
    fn test_variables() {
        let expr = bin(
            bin(Expr::Ident("a"), BinaryOp::Mul, Expr::Num(2)),
            BinaryOp::Add,
            Expr::Ident("b"),
        );

        let mut env = HashMap::new();
        env.insert("a", 3);
        env.insert("b", 5);

        assert_eq!(expr.eval(&env).unwrap(), 11);
    }

    #[test]
    fn test_unknown_ident() {
        let expr = bin(Expr::Ident("x"), BinaryOp::Add, Expr::Num(1));

        assert!(matches!(
            expr.eval(&HashMap::new()),
            Err(EvalError::UnknownIdent("x"))
        ));
    }

    #[test]
    fn test_div_by_zero() {
        let expr = bin(Expr::Num(1), BinaryOp::Div, Expr::Num(0));

        assert!(matches!(
            expr.eval(&HashMap::new()),
            Err(EvalError::DivByZero)
        ));
    }

    #[test]
    fn test_partial_eval() {
        // a + 1 * 2
        let expr = bin(
            Expr::Ident("a"),
            BinaryOp::Add,
            bin(Expr::Num(1), BinaryOp::Mul, Expr::Num(2)),
        );

        let mut env = HashMap::new();
        env.insert("a", 3);

        assert_debug_snapshot!(expr.partial_eval(&env), @"
        Ok(
            (
                Num(
                    5,
                ),
                {},
            ),
        )
        ");
    }

    #[test]
    fn test_partial_eval_unknown() {
        // a + b * 2 + c
        let expr = bin(
            bin(
                Expr::Ident("a"),
                BinaryOp::Add,
                bin(Expr::Ident("b"), BinaryOp::Mul, Expr::Num(2)),
            ),
            BinaryOp::Add,
            Expr::Ident("c"),
        );

        let mut env = HashMap::new();
        env.insert("a", 3);
        env.insert("c", 5);

        assert_debug_snapshot!(expr.partial_eval(&env), @r#"
        Ok(
            (
                Binary {
                    lhs: Binary {
                        lhs: Num(
                            3,
                        ),
                        op: Add,
                        rhs: Binary {
                            lhs: Ident(
                                "b",
                            ),
                            op: Mul,
                            rhs: Num(
                                2,
                            ),
                        },
                    },
                    op: Add,
                    rhs: Num(
                        5,
                    ),
                },
                {
                    "b",
                },
            ),
        )
        "#);
    }
}
