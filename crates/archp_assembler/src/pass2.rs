use anyhow::{Result, anyhow};

use crate::{
    assembler::{Context, Instr, Line},
    instructions::INSTRUCTIONS,
    pseudo_instructions::PSEUDO_INSTRUCTIONS,
};

/// Pass 3
///
/// 1. Expand macro-instructions.
/// 2. Encode assembly instructions into machine code.
pub struct Pass2<'ctx, 'src> {
    context: &'ctx mut Context<'src>,
}

impl<'ctx, 'src> Pass2<'ctx, 'src> {
    pub fn new(context: &'ctx mut Context<'src>) -> Self {
        Pass2 { context }
    }

    pub fn run(&self, processed: Vec<Line<'src>>) -> Result<(Vec<u32>, Vec<Line<'src>>)> {
        let mut codes = Vec::with_capacity(processed.len());
        let mut lines = Vec::with_capacity(processed.len());

        for (idx, (instr, info)) in processed.into_iter().enumerate() {
            let (original_idx, original_line) = info.original_line;

            let (code, instr) = self.line_handler(idx, instr).map_err(|e| {
                anyhow!(
                    "Error encoding line {}: '{}' ({})",
                    original_idx + 1,
                    original_line,
                    e
                )
            })?;

            codes.push(code);
            lines.push((instr, info));
        }

        Ok((codes, lines))
    }

    fn line_handler(&self, idx: usize, instr: Instr<'src>) -> Result<(u32, Instr<'src>)> {
        let pc = (idx * 4) as u32;

        let (name, ops) = instr;

        let (name, ops) = if let Some(ps_instr) = PSEUDO_INSTRUCTIONS.get(name) {
            ps_instr
                .expand(self.context, pc, name, &ops)
                .map_err(|e| anyhow!("Error expanding pseudo-instruction '{}': {}", name, e))?
        } else {
            (name, ops)
        };

        let code = INSTRUCTIONS
            .get(name)
            .ok_or_else(|| anyhow!("Unknown instruction: '{}'", name))?
            .encode(self.context, pc, &ops)?;

        Ok((code, (name, ops)))
    }
}
