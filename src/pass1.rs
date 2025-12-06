use std::collections::HashMap;

use anyhow::{Result, anyhow, bail};

use crate::pseudo_instructions::PSEUDO_INSTRUCTIONS;

/// Pass 1
///
/// 1. Record constants and labels.
/// 2. Handle immediate values.
/// 3. Expand pseudo-instructions.
/// 4. Build a mapping between new lines and the original lines.
/// 5. Substitute constants.
pub struct Pass1<'a> {
    constants: HashMap<&'a str, &'a str>,
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
                    self.labels.insert(label, pc.to_string());

                    if tokens.len() == 1 {
                        continue;
                    }

                    (&raw_line[label.len() + 1..], &tokens[1..])
                }
                None => (raw_line, tokens.as_ref()),
            };

            let (name, cond) = if let Some(idx) = tokens[0].find('.') {
                (&tokens[0][..idx], &tokens[0][idx..])
            } else {
                (tokens[0], "")
            };

            let operands = tokens[1..]
                .iter()
                .map(|e| {
                    if let Some(&value) = self.constants.get(e) {
                        value
                    } else {
                        e
                    }
                })
                .collect::<Vec<_>>();

            let mut expanded = Vec::new();

            if let Some(pseudo) = PSEUDO_INSTRUCTIONS.get(name) {
                expanded.extend(pseudo.expand(&operands).map_err(|e| {
                    anyhow!(
                        "Error expanding pseudo-instruction at line {}: '{}' ({})",
                        orig_idx + 1,
                        raw_line,
                        e,
                    )
                })?);
            } else {
                expanded.push((name, operands.iter().map(|e| e.to_string()).collect()));
            }

            for (ex_name, ex_ops) in expanded {
                let mut line = Vec::new();
                line.push(ex_name.to_string() + cond);
                line.extend(ex_ops);

                self.pc_to_original.push((orig_idx, raw_line.trim()));
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
