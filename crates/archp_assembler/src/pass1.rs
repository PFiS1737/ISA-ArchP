use anyhow::{Result, anyhow, bail};

use crate::{
    assembler::{Context, Instr},
    macro_instructions::MACRO_INSTRUCTIONS,
    parser::{grammar::parse_source, types::grammar::Line},
};

pub struct Pass1<'ctx, 'src> {
    context: &'ctx mut Context<'src>,
}

impl<'ctx, 'src> Pass1<'ctx, 'src> {
    pub fn new(context: &'ctx mut Context<'src>) -> Self {
        Self { context }
    }

    pub fn run(&mut self, source: &'src str) -> Result<Vec<Instr<'src>>> {
        let mut processed = Vec::new();

        let source = parse_source(source).map_err(|e| anyhow!("Error parsing source: {}", e))?;

        for line in source.lines {
            let pc = processed.len() * 4;

            match line {
                Line::Label(label) => {
                    self.context.labels.insert(label, pc);
                },
                Line::Instr {
                    name,
                    operands,
                    line,
                } => {
                    // TODO: remove this
                    if name == ".const" {
                        match line
                            .1
                            .split_once(char::is_whitespace)
                            .unwrap()
                            .1
                            .split_once(',')
                        {
                            Some((name, value)) => {
                                let name = name.trim();
                                let value = value.trim();

                                if name.is_empty() || value.is_empty() {
                                    bail!(
                                        "Invalid constant declaration at line {}: '{}'",
                                        line.0,
                                        line.1
                                    );
                                }

                                // TODO: eval expression at here
                                self.context.constants.insert(name, value);
                            },
                            None => bail!(
                                "Invalid constant declaration at line {}: '{}'",
                                line.0,
                                line.1
                            ),
                        };

                        continue;
                    }

                    if !self.context.settings.disable_macro
                        && let Some(mc_instr) = MACRO_INSTRUCTIONS.get(name)
                        && let Some(expanded) = mc_instr
                            .expand(self.context, pc as u32, name, &operands)
                            .map_err(|e| {
                                anyhow!(
                                    "Error expanding macro-instruction at line {}: '{}' ({})",
                                    line.0,
                                    line.1,
                                    e
                                )
                            })?
                    {
                        for instr in expanded {
                            processed.push(instr);
                            self.context.source_map.push(line);
                        }
                    } else {
                        processed.push((name, operands));
                        self.context.source_map.push(line);
                    };
                },
            }
        }

        Ok(processed)
    }
}
