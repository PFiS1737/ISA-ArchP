use std::collections::HashMap;

use anyhow::{Result, anyhow};

use crate::{instructions::INSTRUCTIONS, macro_instructions::MACRO_INSTRUCTIONS};

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

    pub fn run(&mut self, processed_lines: Vec<Vec<&'a str>>) -> Result<(Vec<u32>, Vec<String>)> {
        let mut codes = Vec::new();
        let mut displays = Vec::new();

        for (addr, line) in processed_lines.iter().enumerate() {
            let (original_idx, original_line) = self.addr_to_original[addr];

            let encoded = self.line_handler(line).map_err(|e| {
                anyhow!(
                    "Error encoding line {}: '{}' ({})",
                    original_idx + 1,
                    original_line,
                    e
                )
            })?;

            // FIXME: perf

            for (_, addr_str) in self.labels.iter_mut() {
                let addr_num = addr_str.parse::<usize>().unwrap();
                if addr_num > addr {
                    *addr_str = (addr_num + encoded.len() - 1).to_string();
                }
            }

            for (mut display, code) in encoded {
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
                displays.push(display.clone());
            }
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

    fn line_handler(&self, line: &[&'a str]) -> Result<Vec<(String, u32)>> {
        let (name, cond) = if let Some((name, cond)) = line[0].split_once('.') {
            (name, Some(cond))
        } else {
            (line[0], None)
        };

        let operands = &line[1..]
            .iter()
            .map(|e| {
                if let Some(label_addr) = self.labels.get(e) {
                    label_addr.as_str()
                } else {
                    e
                }
            })
            .collect::<Vec<_>>();

        let expanded = if let Some(mc_instr) = MACRO_INSTRUCTIONS.get(name)
            && let Some(expanded) = mc_instr
                .expand(operands)
                .map_err(|e| anyhow!("Error expanding macro-instruction '{}': {}", name, e))?
        {
            expanded
        } else {
            vec![(name, operands.iter().map(|e| e.to_string()).collect())]
        };

        let mut codes = Vec::new();

        for (name, ops) in expanded {
            let instr = INSTRUCTIONS
                .get(name)
                .ok_or_else(|| anyhow!("Unknown instruction: '{}'", name))?;

            codes.push((
                format!(
                    "{name}{} {}",
                    cond.map(|e| format!(".{e}")).unwrap_or("".to_string()),
                    ops.join(" ")
                )
                .trim()
                .to_string(),
                instr.encode(cond, &ops.iter().map(|e| e.as_str()).collect::<Vec<_>>())?,
            ));
        }

        Ok(codes)
    }
}
