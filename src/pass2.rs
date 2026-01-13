use anyhow::{Result, anyhow};

use crate::{
    assembler::{Context, Line},
    instructions::INSTRUCTIONS,
    pseudo_instructions::PSEUDO_INSTRUCTIONS,
    utils::fmt_line,
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

    pub fn run(&self, processed: Vec<Line<'src>>) -> Result<(Vec<u32>, Vec<String>)> {
        let mut codes = Vec::new();
        let mut displays = Vec::new();

        for (idx, line) in processed.into_iter().enumerate() {
            let instr_info = &self.context.instr_info[idx];

            let (original_idx, original_line) = instr_info.original_line;

            let (code, mut display) = self.line_handler(idx, line).map_err(|e| {
                anyhow!(
                    "Error encoding line {}: '{}' ({})",
                    original_idx + 1,
                    original_line,
                    e
                )
            })?;

            if display != original_line {
                display = format!("{display}\t[{original_line}]");
            } else {
                display += "\t";
            }

            if let Some(label_name) = instr_info.label_name {
                display = format!("{display}\t<label: {label_name}>");
            } else {
                display += "\t";
            }

            codes.push(code);
            displays.push(display);
        }

        Ok((codes, displays))
    }

    fn line_handler(&self, idx: usize, line: Line) -> Result<(u32, String)> {
        let pc = (idx * 4) as u32;

        let (name, operands) = line;

        let (name, ops) = if let Some(ps_instr) = PSEUDO_INSTRUCTIONS.get(name) {
            ps_instr
                .expand(self.context, pc, &operands)
                .map_err(|e| anyhow!("Error expanding pseudo-instruction '{}': {}", name, e))?
        } else {
            (name, operands)
        };

        let code = INSTRUCTIONS
            .get(name)
            .ok_or_else(|| anyhow!("Unknown instruction: '{}'", name))?
            .encode(self.context, pc, &ops)?;

        Ok((code, fmt_line(name, ops)))
    }
}
