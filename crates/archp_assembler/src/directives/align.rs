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
    let (bytes, value, max) = matches(ctx, ops)?;

    let bytes = 2_i64.pow(bytes as u32);

    align(ctx, bytes, value, max)
};

const F2: HandlerFn = |ctx, ops| {
    let (bytes, value, max) = matches(ctx, ops)?;

    align(ctx, bytes, value, max)
};

fn matches<'a>(ctx: &mut Context<'a>, ops: &[DirectiveOperand<'a>]) -> Result<(i64, i64, i64)> {
    let mut it = ops.iter().map(|op| op.as_evaluated(ctx));

    let bytes = match it.next().transpose()? {
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

    Ok((bytes, value, max))
}

fn align(ctx: &mut Context, bytes: i64, value: i64, max: i64) -> Result<()> {
    if bytes > max {
        return Ok(());
    }

    let pc_bytes = ctx.codes.len() as i64 * 4;

    let aligned_pc = (pc_bytes + bytes - 1) / bytes * bytes;

    let pad_bytes = aligned_pc - pc_bytes;
    let pad_instrs = pad_bytes / 4;

    for _ in 0..pad_instrs {
        ctx.add_code(value as u32, None);
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
        align(&mut ctx, 2_i64.pow(5), 0, i64::MAX).unwrap();
        ctx.add_code(0x12345678, None);
        align(&mut ctx, 2_i64.pow(4), 0, i64::MAX).unwrap();
        ctx.add_code(0x12345678, None);
        align(&mut ctx, 2_i64.pow(3), 0, i64::MAX).unwrap();
        ctx.add_code(0x12345678, None);

        assert_debug_snapshot!(ctx.codes, @"
        [
            305419896,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            305419896,
            0,
            0,
            0,
            305419896,
            0,
            305419896,
        ]
        ");
    }
}
