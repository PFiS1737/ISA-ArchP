use anyhow::{Result, anyhow, bail};

use crate::{
    assembler::{Context, Line},
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

    pub fn run(&mut self, source_lines: &'src [String]) -> Result<Vec<Line<'src>>> {
        let mut processed = Vec::new();

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
                    self.context.constants.insert(name, value);
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
                    let pc = processed.len();
                    self.context.labels.insert(label, pc);

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

            let ops = tokens[1..].iter().map(|e| (*e).into()).collect::<Vec<_>>();

            let mut lines = Vec::new();

            if !self.context.settings.disable_macro
                && let Some(mc_instr) = MACRO_INSTRUCTIONS.get(name)
                && let Some(expanded) =
                    mc_instr
                        .expand(self.context, name, cond, &ops)
                        .map_err(|e| {
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
                self.context
                    .addr_to_original
                    .push((orig_idx, raw_line.trim()));
                processed.push(line);
            }
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
