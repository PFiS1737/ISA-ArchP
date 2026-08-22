use crate::directives::{HandlerFn, directive};

directive! {
    pub Byte {
        name: ".byte",
        handler: F1,
    }
}

const F1: HandlerFn = |ctx, ops| {
    for op in ops {
        let byte = op.cast_absolute(ctx)?;
        ctx.add_byte(byte as u8);
    }

    Ok(())
};

directive! {
    pub Byte2 {
        name: ".2byte",
        handler: F2,
    }
}

directive! {
    pub Half {
        name: ".half",
        handler: F2,
    }
}

directive! {
    pub Short {
        name: ".short",
        handler: F2,
    }
}

const F2: HandlerFn = |ctx, ops| {
    for op in ops {
        let byte = op.cast_absolute(ctx)?;
        ctx.add_half(byte as u16);
    }

    Ok(())
};

directive! {
    pub Byte4 {
        name: ".4byte",
        handler: F4,
    }
}
directive! {
    pub Word {
        name: ".word",
        handler: F4,
    }
}
directive! {
    pub Long {
        name: ".long",
        handler: F4,
    }
}

const F4: HandlerFn = |ctx, ops| {
    for op in ops {
        let byte = op.cast_absolute(ctx)?;
        ctx.add_word(byte as u32);
    }

    Ok(())
};
