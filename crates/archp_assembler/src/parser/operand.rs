use nom::{Parser, bytes::complete::take_until, character::complete::char, sequence::delimited};
use smallvec::SmallVec;

use crate::{
    operand::Operand,
    parser::{Result, expression::expr, ident, parens, types::expression::Expr},
};

fn string_literal(input: &str) -> Result<'_, &str> {
    delimited(char('"'), take_until("\""), char('"')).parse(input)
}

fn unwrap_expr(e: Expr) -> Operand {
    match e {
        Expr::Num(n) => Operand::Num(n),
        Expr::Ident(s) => Operand::Ident(s),
        _ => Operand::Expr(e), // TODO: eval constants
    }
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
    out.push(unwrap_expr(expr));

    // case 4: expr(ident)
    let Ok((input, ident)) = parens(ident).parse(input) else {
        return Ok((input, ()));
    };

    out.insert(out.len() - 1, Operand::Ident(ident));

    Ok((input, ()))
}
