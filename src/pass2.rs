use std::collections::HashMap;

use anyhow::{Result, anyhow};
use indexmap::IndexMap;

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

    pub fn run(&self, processed_lines: Vec<Vec<&'a str>>) -> Result<(Vec<u32>, Vec<String>)> {
        let mut codes = Vec::new();
        let mut displays = Vec::new();

        for (pc, line) in processed_lines.iter().enumerate() {
            let (original_idx, original_line) = self.pc_to_original[pc];

            let (encoded, used_consts) = self.line_handler(line).map_err(|e| {
                anyhow!(
                    "Error encoding line {}: '{}' ({})",
                    original_idx + 1,
                    original_line,
                    e
                )
            })?;

            let mut display = line.join(" ");

            if display != original_line {
                display = format!("{display}\t({original_line})");
            } else {
                display += "\t";
            }

            if !used_consts.is_empty() {
                display = format!(
                    "{display}\t[ {} ]",
                    used_consts
                        .iter()
                        .map(|(name, value)| format!("{name} = {value}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            } else {
                display += "\t";
            }

            if let Some(label_name) = self.find_label_for_pc(pc) {
                display = format!("{display}\t<label: {label_name}>");
            } else {
                display += "\t";
            }

            codes.push(encoded);
            displays.push(display);
        }

        Ok((codes, displays))
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

    fn line_handler(&self, line: &[&'a str]) -> Result<(u32, IndexMap<&'a str, &'a str>)> {
        let (name, cond) = if let Some((name, cond)) = line[0].split_once('.') {
            (name, Some(cond))
        } else {
            (line[0], None)
        };

        let instr = INSTRUCTIONS
            .get(name)
            .ok_or_else(|| anyhow!("Unknown instruction: '{}'", name))?;

        let mut used_consts = IndexMap::new();
        let code = instr.encode(
            cond,
            &(line[1..]
                .iter()
                .map(|e| {
                    if let Some(&const_value) = self.constants.get(e) {
                        used_consts.insert(*e, const_value);
                        const_value
                    } else if let Some(label_addr) = self.labels.get(e) {
                        label_addr.as_str()
                    } else {
                        e
                    }
                })
                .collect::<Vec<_>>()),
        )?;

        Ok((code, used_consts))
    }
}
