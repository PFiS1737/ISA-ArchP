use anyhow::{Result, anyhow};

use crate::{
    assembler::{Context, Instr},
    directives::DIRECTIVES,
    macro_instructions::MACRO_INSTRUCTIONS,
    parser::{line::parse_line, types::line::Line},
    pseudo_instructions::PSEUDO_INSTRUCTIONS,
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

        for (line_idx, line) in source.lines().enumerate() {
            for line in parse_line(self.context, line_idx + 1, line)? {
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
                        if let Some(dire) = DIRECTIVES.get(name) {
                            dire.handle(self.context, &operands).map_err(|e| {
                                anyhow!(
                                    "Error handling directive at line {}: '{}' ({})",
                                    line.0,
                                    line.1,
                                    e
                                )
                            })?;
                            continue;
                        }

                        let instrs = if !self.context.settings.disable_macro
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
                                })? {
                            expanded
                        } else {
                            vec![(name, operands)]
                        };

                        for (name, ops) in instrs.into_iter() {
                            if let Some(ps_instr) = PSEUDO_INSTRUCTIONS.get(name) {
                                let expanded =
                                    ps_instr.expand(self.context, &ops).map_err(|e| {
                                        anyhow!(
                                            "Error expanding pseudo-instruction '{}': {}",
                                            name,
                                            e
                                        )
                                    })?;
                                for instr in expanded {
                                    processed.push(instr);
                                    self.context.source_map.push(line);
                                }
                            } else {
                                processed.push((name, ops));
                                self.context.source_map.push(line);
                            }
                        }
                    },
                }
            }
        }

        Ok(processed)
    }
}
