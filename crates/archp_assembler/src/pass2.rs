use anyhow::Result;

use crate::context::Context;

pub struct Pass2<'ctx, 'src> {
    context: &'ctx mut Context<'src>,
}

impl<'ctx, 'src> Pass2<'ctx, 'src> {
    pub fn new(context: &'ctx mut Context<'src>) -> Self {
        Pass2 { context }
    }

    pub fn run(&mut self) -> Result<()> {
        for reloc in &self.context.relocations {
            let addr = self
                .context
                .labels
                .get(reloc.label)
                .ok_or_else(|| anyhow::anyhow!("Undefined label: {}", reloc.label))?;

            let addr = *addr as i64 + reloc.addend;

            let code = &mut self.context.codes[reloc.offset / 4];

            reloc
                .instr
                .apply_relocation(code, addr, reloc.offset as u32)?;
        }

        Ok(())
    }
}
