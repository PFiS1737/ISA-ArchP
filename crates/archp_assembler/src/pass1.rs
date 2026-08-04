use anyhow::{Result, anyhow, bail};
use smallvec::SmallVec;

use crate::{
    assembler::{Context, Line, LineInfo},
    macro_instructions::MACRO_INSTRUCTIONS,
};

/// Pass 1
///
/// 1. Record constants and labels.
/// 2. Expand macro-instructions.
/// 3. Build a mapping between new lines and the original lines.
pub struct Pass1<'ctx, 'src> {
    context: &'ctx mut Context<'src>,
}

impl<'ctx, 'src> Pass1<'ctx, 'src> {
    pub fn new(context: &'ctx mut Context<'src>) -> Self {
        Self { context }
    }

    pub fn run(
        &mut self,
        source_lines: impl IntoIterator<Item = &'src str>,
    ) -> Result<Vec<Line<'src>>> {
        let mut processed = Vec::new();

        let mut in_const_zone = true;
        let mut current_label = None;

        for (orig_idx, raw_line) in source_lines.into_iter().enumerate() {
            let raw_line = raw_line.trim();
            if raw_line.is_empty() {
                continue;
            }

            let raw_line = strip_comment(raw_line).trim();
            if raw_line.is_empty() {
                continue;
            }

            let (name, remain) = match raw_line.split_once(char::is_whitespace) {
                Some(pair) => pair,
                None => (raw_line, ""),
            };

            // TODO: remove this after implementing directive .eqv
            if name == "const" {
                if in_const_zone {
                    match remain.split_once('=') {
                        Some((name, value)) => {
                            let name = name.trim();
                            let value = value.trim();

                            if name.is_empty() || value.is_empty() {
                                bail!(
                                    "Invalid constant declaration at line {}: '{}'",
                                    orig_idx + 1,
                                    raw_line
                                );
                            }

                            self.context.constants.insert(name, value);
                        },
                        None => bail!(
                            "Invalid constant declaration at line {}: '{}'",
                            orig_idx + 1,
                            raw_line
                        ),
                    };

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

            let pc = processed.len() * 4;

            let (raw_line, name, remain) = match name.strip_suffix(':') {
                Some(label) => {
                    self.context.labels.insert(label, pc);

                    current_label = Some(label);

                    if remain.is_empty() {
                        continue;
                    }

                    let line = raw_line[label.len() + 1..].trim();

                    match line.split_once(char::is_whitespace) {
                        Some(pair) => (line, pair.0, pair.1),
                        None => (line, line, ""),
                    }
                },
                None => (raw_line, name, remain),
            };

            let ops = remain
                .split(',')
                .filter(|e| !e.is_empty())
                .map(|e| e.trim().into())
                .collect::<SmallVec<_>>();

            if !self.context.settings.disable_macro
                && let Some(mc_instr) = MACRO_INSTRUCTIONS.get(name)
                && let Some(expanded) = mc_instr
                    .expand(self.context, pc as u32, name, &ops)
                    .map_err(|e| {
                        anyhow!(
                            "Error expanding macro-instruction at line {}: '{}' ({})",
                            orig_idx + 1,
                            raw_line,
                            e
                        )
                    })?
            {
                for instr in expanded {
                    processed.push((instr, LineInfo {
                        original_line: (orig_idx, raw_line),
                        label_name: current_label,
                    }));

                    current_label = None;
                }
            } else {
                processed.push(((name, ops), LineInfo {
                    original_line: (orig_idx, raw_line),
                    label_name: current_label,
                }));

                current_label = None;
            };
        }

        Ok(processed)
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
