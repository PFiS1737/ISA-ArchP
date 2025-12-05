use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::pseudo_instructions::PSEUDO_INSTRUCTIONS;

/// Pass 1
///
/// 1. Record constants and labels.
/// 2. Handle immediate values.
/// 3. Expand pseudo-instructions.
/// 4. Build a mapping between new lines and the original lines.
pub struct Pass1<'a> {
    pub constants: HashMap<&'a str, &'a str>,
    pub labels: HashMap<&'a str, String>,
    pub pc_to_original: Vec<(usize, &'a str)>,
    pub processed: Vec<Vec<String>>,
}

impl<'a> Pass1<'a> {
    pub fn new() -> Self {
        Self {
            constants: HashMap::new(),
            labels: HashMap::new(),
            pc_to_original: Vec::new(),
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

            // strip comments (; or #)
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
                    // expected form: const NAME VALUE
                    if tokens.len() != 3 {
                        bail!("Malformed const at line {}: {}", orig_idx + 1, raw_line);
                    }
                    let [name, value, ..] = tokens[1..] else {
                        unreachable!()
                    };
                    self.constants.insert(name, value);
                    continue;
                }

                bail!(
                    "Constants must be declared at the start of file (line {}): {}",
                    orig_idx + 1,
                    raw_line
                )
            }

            // first non-const line -> close const zone forever
            if in_const_zone {
                in_const_zone = false;
            }

            if tokens.len() == 1
                && let Some(label) = tokens[0].strip_suffix(":")
            {
                let pc = self.processed.len();
                self.labels.insert(label, pc.to_string());
                continue;
            }

            if tokens[0].ends_with(':') {
                bail!(
                    "Labels must be on their own line (line {}): {}",
                    orig_idx + 1,
                    raw_line
                );
            }

            let (name, cond) = if let Some(idx) = tokens[0].find(".") {
                (&tokens[0][..idx], &tokens[0][idx..])
            } else {
                (tokens[0], "")
            };

            let operands = &tokens[1..];

            if let Some(pseudo) = PSEUDO_INSTRUCTIONS.get(name) {
                let expanded = pseudo.expand(operands)?;
                for (ex_name, ex_ops) in expanded {
                    self.emit(ex_name.to_string() + cond, ex_ops, orig_idx, raw_line)?;
                }
            } else {
                self.emit(
                    tokens[0].to_string(),
                    operands.iter().map(|e| e.to_string()).collect(),
                    orig_idx,
                    raw_line,
                )?;
            }
        }

        Ok(())
    }

    fn emit(
        &mut self,
        name: String,
        operands: Vec<String>,
        orig_idx: usize,
        raw_line: &'a str,
    ) -> Result<()> {
        let mut line = Vec::new();
        line.push(name);
        line.extend(operands);

        self.pc_to_original.push((orig_idx, raw_line.trim()));
        self.processed.push(line);

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
