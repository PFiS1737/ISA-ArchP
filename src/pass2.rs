use anyhow::{Result, anyhow};

use crate::{
    assembler::{Context, Line},
    instructions::INSTRUCTIONS,
    operand::OperandValue,
    pseudo_instructions::PSEUDO_INSTRUCTIONS,
    utils::fmt_line,
};

/// Pass 3
///
/// 1. Substitute label addresses.
/// 2. Expand macro-instructions.
/// 3. Encode assembly instructions into machine code.
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

        for (addr, line) in processed.into_iter().enumerate() {
            let (original_idx, original_line) = self.context.addr_to_original[addr];

            let (code, mut display) = self.line_handler(line).map_err(|e| {
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

            if let Some(label_name) = self.context.labels.get_by_right(&codes.len()) {
                display = format!("{display}\t<label: {label_name}>");
            } else {
                display += "\t";
            }

            codes.push(code);
            displays.push(display);
        }

        Ok((codes, displays))
    }

    fn line_handler(&self, line: Line) -> Result<(u32, String)> {
        let (name, cond, operands) = line;

        let operands = operands
            .into_iter()
            .map(|e| {
                if let Some(s) = e.as_str()
                    && let Some(&label_addr) = self.context.labels.get_by_left(s)
                {
                    OperandValue::Unsigned(label_addr.try_into().unwrap()) // WARN: unsafe
                } else {
                    e
                }
            })
            .collect::<Vec<_>>();

        let (name, ops) = if let Some(ps_instr) = PSEUDO_INSTRUCTIONS.get(name) {
            ps_instr
                .expand(&operands)
                .map_err(|e| anyhow!("Error expanding pseudo-instruction '{}': {}", name, e))?
        } else {
            (name, operands)
        };

        let code = INSTRUCTIONS
            .get(name)
            .ok_or_else(|| anyhow!("Unknown instruction: '{}'", name))?
            .encode(cond, &ops)?;

        Ok((code, fmt_line(name, cond, ops)))
    }
}
