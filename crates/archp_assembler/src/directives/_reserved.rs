use crate::directives::directive;

// INFO: See https://github.com/riscv-non-isa/riscv-asm-manual/blob/main/src/asm-manual.adoc
//           search: 'accepted for source compatibility'

directive! {
    pub Ident {
        name: ".ident",
        handler: |_, _| Ok(()),
    }
}

directive! {
    pub Size {
        name: ".size",
        handler: |_, _| Ok(()),
    }
}

directive! {
    pub Type {
        name: ".type",
        handler: |_, _| Ok(()),
    }
}
