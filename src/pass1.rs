use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};

use crate::{macro_instructions::MACRO_INSTRUCTIONS, operand::OperandValue};

/// Pass 1
///
/// 1. Record constants and labels.
/// 2. Expand macro-instructions.
/// 3. Substitute constants.
/// 4. Build a mapping between new lines and the original lines.
pub struct Pass1<'a> {
    disable_macro: bool,
    constants: HashMap<&'a str, &'a str>,
    pub labels: HashMap<&'a str, usize>,
    pub addr_to_original: Vec<(usize, &'a str)>,
    pub processed: Vec<(&'a str, Option<&'a str>, Vec<OperandValue<'a>>)>,
}

impl<'a> Pass1<'a> {
    pub fn new(disable_macro: bool) -> Self {
        Self {
            disable_macro,
            constants: HashMap::new(),
            labels: HashMap::new(),
            addr_to_original: Vec::new(),
            processed: Vec::new(),
        }
    }

    pub fn run(&mut self, source_lines: &'a [String]) -> Result<()> {
        let mut in_const_zone = true;

        for (orig_idx, raw_line) in source_lines.iter().enumerate() {
            let raw_line = raw_line.trim();
            if raw_line.is_empty() {
                continue;
            }

            let raw_line = strip_comment(raw_line).trim();
            if raw_line.is_empty() {
                continue;
            }

            let tokens = raw_line.split_whitespace().collect::<Vec<_>>();
            if tokens.is_empty() {
                unreachable!()
            }

            if tokens[0] == "const" {
                if in_const_zone {
                    if tokens.len() != 3 {
                        bail!("Malformed const at line {}: '{}'", orig_idx + 1, raw_line);
                    }
                    let [name, value, ..] = tokens[1..] else {
                        unreachable!()
                    };
                    self.constants.insert(name, value);
                    continue;
                }

                bail!(
                    "Constants must be declared at the start of file (line {}): '{}'",
                    orig_idx + 1,
                    raw_line
                );
            }

            if in_const_zone {
                in_const_zone = false;
            }

            let (raw_line, tokens) = match tokens[0].strip_suffix(':') {
                Some(label) => {
                    let pc = self.processed.len();
                    self.labels.insert(label, pc);

                    if tokens.len() == 1 {
                        continue;
                    }

                    (&raw_line[label.len() + 1..], &tokens[1..])
                }
                None => (raw_line, tokens.as_ref()),
            };

            let (name, cond) = if let Some((name, cond)) = tokens[0].split_once('.') {
                (name, Some(cond))
            } else {
                (tokens[0], None)
            };

            let ops = tokens[1..]
                .iter()
                .map(|e| {
                    OperandValue::from(if let Some(&value) = self.constants.get(e) {
                        value
                    } else {
                        e
                    })
                })
                .collect::<Vec<_>>();

            let mut lines = Vec::new();

            if !self.disable_macro
                && let Some(mc_instr) = MACRO_INSTRUCTIONS.get(name)
                && let Some(expanded) = mc_instr.expand(cond, &ops).map_err(|e| {
                    anyhow!(
                        "Error expanding macro-instruction at line {}: '{}' ({})",
                        orig_idx + 1,
                        raw_line,
                        e
                    )
                })?
            {
                lines.extend(expanded);
            } else {
                lines.push((name, cond, ops));
            }

            for line in lines {
                self.addr_to_original.push((orig_idx, raw_line.trim()));
                self.processed.push(line);
            }
        }

        Ok(())
    }
}

fn strip_comment(s: &str) -> &str {
    if let Some(idx) = s.find(';') {
        &s[..idx]
    } else if let Some(idx) = s.find('#') {
        &s[..idx]
    } else {
        s
    }
}
