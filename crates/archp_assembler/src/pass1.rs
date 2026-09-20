use anyhow::{Result, anyhow};
use smallvec::SmallVec;

use crate::{
    assembler::{Instr, Line},
    context::Context,
    directives::DIRECTIVES,
    instruction::Instruction,
    operand::Operand,
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
                // INFO: Make sure the text section is aligned to 4 bytes before adding labels
                // FIXME: Don't align if it is labeled on a data
                self.context.align4();

                let pc = self.context.text.len();
                self.context.labels.insert(label, pc);
            },
            Line::Directive {
                name,
                operands,
                line,
            } => {
                DIRECTIVES
                    .get(name)
                    .ok_or(anyhow!("Unknown directive: '{}'", name))?
                    .handle(self.context, &operands)
                    .map_err(|e| {
                        anyhow!(
                            "Error handling directive at line {}: '{}' ({})",
                            line.0,
                            line.1,
                            e
                        )
                    })?;
            },
            Line::Instruction {
                name,
                operands,
                line,
            } => {
                // INFO: Make sure the text section is aligned to 4 bytes before adding instructions
                self.context.align4();

                #[cfg(feature = "macros")]
                {
                    if let Some(mc_instr) = crate::macro_instructions::MACRO_INSTRUCTIONS.get(name)
                        && let Some(expanded) = mc_instr
                            .expand(self.context, name, &operands)
                            .map_err(|e| {
                                anyhow!(
                                    "Error expanding macro-instruction at line {}: '{}' ({})",
                                    line.0,
                                    line.1,
                                    e
                                )
                            })?
                    {
                        for (name, ops) in expanded.into_iter() {
                            self.handle_instr(name, ops, line)?;
                        }
                    } else {
                        self.handle_instr(name, operands, line)?;
                    };
                }

                #[cfg(not(feature = "macros"))]
                {
                    self.handle_instr(name, operands, line)?;
                }
            },
        }

        Ok(())
    }

    fn handle_instr(
        &mut self,
        name: &'src str,
        ops: SmallVec<[Operand<'src>; 3]>,
        line: (usize, &'src str),
    ) -> Result<()> {
        if let Some(ps_instr) = PSEUDO_INSTRUCTIONS.get(name) {
            let expanded = ps_instr.expand(self.context, &ops).map_err(|e| {
                anyhow!(
                    "Error expanding pseudo-instruction at line {}: '{}' ({})",
                    line.0,
                    line.1,
                    e
                )
            })?;
            for instr in expanded {
                self.encode_instr(instr)?;
            }
        } else {
            self.encode_instr((name, ops))?;
        }

        Ok(())
    }

    fn encode_instr(&mut self, instr: Instr<'src>) -> Result<()> {
        let (name, ops) = instr;

        let code = Instruction::get_by_name(name)
            .ok_or(anyhow!("Unknown instruction: '{}'", name))?
            .encode(self.context, &ops)?;

        self.context.add_word(code);

        Ok(())
    }
}
