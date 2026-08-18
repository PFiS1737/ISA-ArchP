use anyhow::{Result, anyhow};

use crate::{
    assembler::{Instr, Line},
    context::Context,
    directives::DIRECTIVES,
    instructions::INSTRUCTIONS,
    macro_instructions::MACRO_INSTRUCTIONS,
    parser::parse_line,
    pseudo_instructions::PSEUDO_INSTRUCTIONS,
};

pub struct Pass1<'ctx, 'src> {
    context: &'ctx mut Context<'src>,
}

impl<'ctx, 'src> Pass1<'ctx, 'src> {
    pub fn new(context: &'ctx mut Context<'src>) -> Self {
        Self { context }
    }

    pub fn run(&mut self, source: &'src str) -> Result<()> {
        for (line_idx, line) in source.lines().enumerate() {
            for line in parse_line(self.context, line_idx + 1, line)? {
                self.handle_line(line)?;
            }
        }

        Ok(())
    }

    fn handle_line(&mut self, line: Line<'src>) -> Result<()> {
        match line {
            Line::Label(label) => {
                let pc = self.context.codes.len() * 4;
                self.context.labels.insert(label, pc);
            },
            Line::Directive {
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
                    })?
                }
            },
            Line::Instruction {
                name,
                operands,
                line,
            } => {
                let instrs = if !self.context.settings.disable_macro
                    && let Some(mc_instr) = MACRO_INSTRUCTIONS.get(name)
                    && let Some(expanded) =
                        mc_instr
                            .expand(self.context, name, &operands)
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
                        let expanded = ps_instr.expand(self.context, &ops).map_err(|e| {
                            anyhow!("Error expanding pseudo-instruction '{}': {}", name, e)
                        })?;
                        for instr in expanded {
                            self.handle_instr(instr)?;
                        }
                    } else {
                        self.handle_instr((name, ops))?;
                    }
                }
            },
        }

        Ok(())
    }

    fn handle_instr(&mut self, instr: Instr<'src>) -> Result<()> {
        let pc = self.context.codes.len() * 4;

        let (name, ops) = instr;

        let code = INSTRUCTIONS
            .get(name)
            .ok_or(anyhow!("Unknown instruction: '{}'", name))?
            .encode(self.context, pc, &ops)?;

        self.context.add_code(code, Some((name, ops)));

        Ok(())
    }
}
