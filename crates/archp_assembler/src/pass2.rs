use anyhow::{Result, anyhow};

use crate::context::Context;

pub struct Pass2<'ctx, 'src> {
    context: &'ctx mut Context<'src>,
}

impl<'ctx, 'src> Pass2<'ctx, 'src> {
    pub fn new(context: &'ctx mut Context<'src>) -> Self {
        Pass2 { context }
    }

    pub fn run(&mut self) -> Result<()> {
        // TODO: optimize this
        let relocations = self.context.relocations.clone();

        for reloc in relocations {
            let addr = self
                .context
                .labels
                .get(reloc.label)
                .ok_or(anyhow!("Undefined label: {}", reloc.label))?;

            let addr = *addr as i64 + reloc.addend;

            let code = self.context.get_code(reloc.offset);

            let code = reloc
                .instr
                .apply_relocation(code, addr, reloc.base as u32)?;

            self.context.set_code(reloc.offset, code);
        }

        Ok(())
    }
}
