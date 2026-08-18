use std::{iter::once, str::CharIndices};

use anyhow::{Result, anyhow, bail};
use nom::AsChar;

use crate::{
    context::Context,
    directives::{HandlerFn, directive},
    operand::DirectiveOperand,
};

directive! {
    pub Ascii {
        name: ".ascii",
        handler: F1,
    }
}

directive! {
    pub Asciz {
        name: ".asciz",
        handler: F2,
    }
}

directive! {
    pub String {
        name: ".string",
        handler: F2,
    }
}

const F1: HandlerFn = |ctx, ops| {
    for op in ops {
        let DirectiveOperand::String(str) = op else {
            bail!("operands mismatch");
        };

        push_str(ctx, str)?;
    }

    Ok(())
};

const F2: HandlerFn = |ctx, ops| {
    for op in ops {
        let DirectiveOperand::String(str) = op else {
            bail!("operands mismatch");
        };

        push_str(ctx, str)?;
        ctx.add_byte(0);
    }

    Ok(())
};

fn push_str(ctx: &mut Context, s: &str) -> Result<()> {
    let mut chars = s.char_indices();

    while let Some((_, c)) = chars.next() {
        if c != '\\' {
            ctx.add_byte(c as u8);
        } else {
            ctx.add_byte(unescape_char(&mut chars, s)?);
        }
    }

    Ok(())
}

fn unescape_char(iter: &mut CharIndices, raw: &str) -> Result<u8> {
    let (i, c) = iter.next().ok_or(anyhow!("unexpected end of string"))?;

    let byte = match c {
        '\'' | '"' | '?' | '\\' => c as u8,
        'a' => b'\x07',
        'b' => b'\x08',
        'f' => b'\x0c',
        'n' => b'\x0a',
        'r' => b'\x0d',
        't' => b'\x09',
        'v' => b'\x0b',
        'x' | 'X' => {
            let hex = iter.take_while(|x| x.1.is_hex_digit());
            let end = hex
                .last()
                .ok_or(anyhow!("expected one or more hex digit after \\x"))?
                .0;

            u8::from_str_radix(&raw[i + 1..=end], 16)?
        },
        c if c.is_oct_digit() => {
            let mut count = 0;
            let oct = once((i, c)).chain(iter.take_while(|x| {
                if count >= 2 {
                    return false;
                }

                if x.1.is_oct_digit() {
                    count += 1;
                    true
                } else {
                    false
                }
            }));

            // INFO: Safe to unwrap because we always have at least one digit
            let end = oct.last().unwrap().0;

            u8::from_str_radix(&raw[i..=end], 8)?
        },
        _ => bail!("invalid escape sequence: \\{}", c),
    };

    Ok(byte)
}
