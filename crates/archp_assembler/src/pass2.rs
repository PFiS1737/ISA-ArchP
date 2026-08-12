use anyhow::{Result, anyhow};

use crate::{
    assembler::{Context, Instr},
    instructions::INSTRUCTIONS,
    pseudo_instructions::PSEUDO_INSTRUCTIONS,
};

pub struct Pass2<'ctx, 'src> {
    context: &'ctx mut Context<'src>,
}

impl<'ctx, 'src> Pass2<'ctx, 'src> {
    pub fn new(context: &'ctx mut Context<'src>) -> Self {
        Pass2 { context }
    }

    pub fn run(&self, processed: Vec<Instr<'src>>) -> Result<(Vec<u32>, Vec<Instr<'src>>)> {
        let mut codes = Vec::with_capacity(processed.len());
        let mut instrs = Vec::with_capacity(processed.len());

        for (idx, instr) in processed.into_iter().enumerate() {
            let (original_idx, original_line) = self.context.source_map[idx];

            let (code, instr) = self.line_handler(idx, instr).map_err(|e| {
                anyhow!(
                    "Error encoding line {}: '{}' ({})",
                    original_idx + 1,
                    original_line,
                    e
                )
            })?;

            codes.push(code);
            instrs.push(instr);
        }

        Ok((codes, instrs))
    }

    fn line_handler(&self, idx: usize, instr: Instr<'src>) -> Result<(u32, Instr<'src>)> {
        let pc = (idx * 4) as u32;

        let (name, ops) = instr;

        let (name, ops) = if let Some(ps_instr) = PSEUDO_INSTRUCTIONS.get(name) {
            ps_instr
                .expand(self.context, pc, &ops)
                .map_err(|e| anyhow!("Error expanding pseudo-instruction '{}': {}", name, e))?
        } else {
            (name, ops)
        };

        let code = INSTRUCTIONS
            .get(name)
            .ok_or(anyhow!("Unknown instruction: '{}'", name))?
            .encode(self.context, pc, &ops)?;

        Ok((code, (name, ops)))
    }
}
