use std::iter::repeat_n;

use anyhow::{Result, bail};

use crate::{
    context::Context,
    directives::{HandlerFn, directive},
    operand::{
        DirectiveOperand::{self, *},
        Operand::*,
    },
};

directive! {
    pub Align {
        name: ".align",
        handler: F1,
    }
}

directive! {
    pub P2align {
        name: ".p2align",
        handler: F1,
    }
}

directive! {
    pub Balign {
        name: ".balign",
        handler: F2,
    }
}

const F1: HandlerFn = |ctx, ops| {
    let (p2, value, max) = matches(ctx, ops)?;

    let align = 2_usize.pow(p2 as u32);

    align_bytes(ctx, align, value as u8, max as usize)
};

const F2: HandlerFn = |ctx, ops| {
    let (align, value, max) = matches(ctx, ops)?;

    align_bytes(ctx, align as usize, value as u8, max as usize)
};

fn matches<'a>(ctx: &mut Context<'a>, ops: &[DirectiveOperand<'a>]) -> Result<(i64, i64, i64)> {
    let mut it = ops.iter().map(|op| op.as_evaluated(ctx));

    let align = match it.next().transpose()? {
        Some(Operand(Num(bytes))) => bytes,
        _ => bail!("operands mismatch"),
    };

    let value = match it.next().transpose()? {
        Some(Operand(Num(value))) => value,
        Some(Empty) | None => 0,
        _ => bail!("operands mismatch"),
    };

    let max = match it.next().transpose()? {
        Some(Operand(Num(max))) => max,
        None => i64::MAX,
        _ => bail!("operands mismatch"),
    };

    if it.next().is_some() {
        bail!("operands mismatch");
    }

    if align < 0 || max < 0 {
        bail!("alignment and max must be non-negative");
    }

    Ok((align, value, max))
}

fn align_bytes(ctx: &mut Context, align: usize, value: u8, max: usize) -> Result<()> {
    let rem = ctx.text.len() % align;

    if rem != 0 {
        let padding = align - rem;

        if padding > max {
            return Ok(());
        }

        ctx.text.reserve(padding);
        ctx.text.extend(repeat_n(value, padding));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    #[test]
    fn test_align() {
        let mut ctx = Context::test();

        ctx.add_code(0x12345678, None);
        align_bytes(&mut ctx, 2_usize.pow(5), 0, usize::MAX).unwrap();
        ctx.add_code(0x12345678, None);
        align_bytes(&mut ctx, 2_usize.pow(4), 0, usize::MAX).unwrap();
        ctx.add_code(0x12345678, None);
        align_bytes(&mut ctx, 2_usize.pow(3), 0, usize::MAX).unwrap();
        ctx.add_code(0x12345678, None);

        assert_debug_snapshot!(ctx.text, @"
        [
            120,
            86,
            52,
            18,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            120,
            86,
            52,
            18,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            120,
            86,
            52,
            18,
            0,
            0,
            0,
            0,
            120,
            86,
            52,
            18,
        ]
        ");
    }
}
