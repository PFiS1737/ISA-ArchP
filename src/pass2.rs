use std::collections::HashMap;

use anyhow::{Result, anyhow};

use crate::{instructions::INSTRUCTIONS, pseudo_instructions::PSEUDO_INSTRUCTIONS};

/// Pass 3
///
/// 1. Substitute label addresses.
/// 2. Expand macro-instructions.
/// 3. Encode assembly instructions into machine code.
pub struct Pass2<'a> {
    labels: HashMap<&'a str, String>,
    addr_to_original: Vec<(usize, &'a str)>,
}

impl<'a> Pass2<'a> {
    pub fn new(labels: HashMap<&'a str, String>, addr_to_original: Vec<(usize, &'a str)>) -> Self {
        Pass2 {
            labels,
            addr_to_original,
        }
    }

    pub fn run(&self, processed_lines: Vec<Vec<&'a str>>) -> Result<(Vec<u32>, Vec<String>)> {
        let mut codes = Vec::new();
        let mut displays = Vec::new();

        for (addr, line) in processed_lines.iter().enumerate() {
            let (original_idx, original_line) = self.addr_to_original[addr];

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

            if let Some(label_name) = self.find_label_for(codes.len()) {
                display = format!("{display}\t<label: {label_name}>");
            } else {
                display += "\t";
            }

            codes.push(code);
            displays.push(display);
        }

        Ok((codes, displays))
    }

    fn find_label_for(&self, pc: usize) -> Option<&'a str> {
        let pc = pc.to_string();
        for (name, addr) in &self.labels {
            if addr == &pc {
                return Some(name);
            }
        }
        None
    }

    fn line_handler(&self, line: &[&'a str]) -> Result<(u32, String)> {
        let (name, cond) = if let Some((name, cond)) = line[0].split_once('.') {
            (name, Some(cond))
        } else {
            (line[0], None)
        };

        let operands = line[1..]
            .iter()
            .map(|e| {
                if let Some(label_addr) = self.labels.get(e) {
                    label_addr.as_str()
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

        let mut display = String::with_capacity(
            name.len()
                + cond.map(|c| 1 + c.len()).unwrap_or(0)
                + if ops.is_empty() { 0 } else { 1 }
                + ops.iter().map(|o| o.len()).sum::<usize>()
                + ops.len().saturating_sub(1),
        );

        display.push_str(name);

        if let Some(c) = cond {
            display.push('.');
            display.push_str(c);
        }

        if !ops.is_empty() {
            for op in ops {
                display.push(' ');
                display.push_str(op);
            }
        }

        Ok((code, display))
    }
}
