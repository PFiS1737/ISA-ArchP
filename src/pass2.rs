use std::collections::HashMap;

use anyhow::{Result, anyhow};

use crate::instructions::INSTRUCTIONS;

/// Pass 3
///
/// Encodes assembly instructions into machine code.
pub struct Pass2<'a> {
    constants: HashMap<&'a str, &'a str>,
    labels: HashMap<&'a str, String>,
    pc_to_original: &'a Vec<(usize, &'a str)>,
}

impl<'a> Pass2<'a> {
    pub fn new(
        constants: HashMap<&'a str, &'a str>,
        labels: HashMap<&'a str, String>,
        pc_to_original: &'a Vec<(usize, &'a str)>,
    ) -> Self {
        Pass2 {
            constants,
            labels,
            pc_to_original,
        }
    }

    pub fn run(&self, processed_lines: Vec<Vec<&'a str>>) -> Result<Vec<(u32, String)>> {
        let mut result = Vec::new();

        for (pc, line) in processed_lines.iter().enumerate() {
            let encoded = self.line_handler(line).map_err(|e| {
                let (original_line_number, original_line) = self.pc_to_original[pc];
                anyhow!(
                    "Error encoding line {}: '{}' ({})",
                    original_line_number + 1,
                    original_line,
                    e
                )
            })?;

            let (_, original_line) = self.pc_to_original[pc];
            let joined = line.join(" ");

            let mut display = if joined == original_line {
                joined
            } else {
                format!("{joined} ({original_line})")
            };

            if let Some(label_name) = self.find_label_for_pc(pc) {
                display = format!("{display} [label: {label_name}]");
            }

            result.push((encoded, display));
        }

        Ok(result)
    }

    fn find_label_for_pc(&self, pc: usize) -> Option<&'a str> {
        let pc_str = pc.to_string();
        for (name, addr) in &self.labels {
            if addr == &pc_str {
                return Some(name);
            }
        }
        None
    }

    fn line_handler(&self, line: &[&'a str]) -> Result<u32> {
        let (name, cond) = if let Some((name, cond)) = line[0].split_once('.') {
            (name, Some(cond))
        } else {
            (line[0], None)
        };

        let instr = INSTRUCTIONS
            .get(name)
            .ok_or_else(|| anyhow!("Unknown instruction: '{}'", name))?;

        instr.encode(
            cond,
            &(line[1..]
                .iter()
                .map(|e| {
                    if let Some(&const_value) = self.constants.get(e) {
                        const_value
                    } else if let Some(label_addr) = self.labels.get(e) {
                        label_addr.as_str()
                    } else {
                        e
                    }
                })
                .collect::<Vec<_>>()),
        )
    }
}
