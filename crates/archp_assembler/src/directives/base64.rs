use anyhow::{Result, bail};
use base64::prelude::*;

use crate::directives::{DirectiveOperand, HandlerFn, directive};

directive! {
    pub Base64 {
        name: ".base64",
        handler: F,
    }
}

const F: HandlerFn = |ctx, ops| {
    for op in ops {
        let DirectiveOperand::String(str) = op else {
            bail!("operands mismatch");
        };

        let len = base64_decoded_len(str)?;

        let slice = ctx.reserve_text(len);

        BASE64_STANDARD.decode_slice(str, slice)?;
    }

    Ok(())
};

fn base64_decoded_len(b64: &str) -> Result<usize> {
    let len = b64.len();

    if len == 0 {
        return Ok(0);
    }

    if !len.is_multiple_of(4) {
        bail!("Invalid base64 length");
    }

    let bytes = b64.as_bytes();

    let padding = match (bytes[len - 1], bytes[len - 2]) {
        (b'=', b'=') => 2,
        (b'=', _) => 1,
        _ => 0,
    };

    let len = (len / 4) * 3 - padding;

    Ok(len)
}
