use anyhow::{Result, anyhow, bail};
use smallvec::SmallVec;

use crate::{
    assembler::{Context, Line as ALine, LineInfo},
    macro_instructions::MACRO_INSTRUCTIONS,
    operand::OperandValue,
    parser::{
        grammar::parse_source,
        types::{
            expression::Expr,
            grammar::{Line, Operand},
        },
    },
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

    pub fn run(&mut self, source: &'src str) -> Result<Vec<ALine<'src>>> {
        let mut processed = Vec::new();

        let mut current_label = None;

        let ast = parse_source(source).map_err(|e| anyhow!("Error parsing source: {}", e))?;

        for line in ast.lines {
            let pc = processed.len() * 4;

            match line {
                Line::Label(label) => {
                    self.context.labels.insert(label, pc);

                    current_label = Some(label);
                },
                Line::Instr {
                    name,
                    operands,
                    line,
                } => {
                    // TODO: remove this after implementing directive .eqv
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

                    // TODO: pass-through operands
                    let ops = operands
                        .iter()
                        .map(|op| match op {
                            Operand::Num(n) => OperandValue::Integer(*n as u32),
                            Operand::Ident(s) => OperandValue::StringSlice(s),
                            Operand::String(_) => unimplemented!("string"),
                            Operand::Expr(expr) => {
                                match expr.partial_eval_with(&|_| None).unwrap().0 {
                                    Expr::Num(n) => OperandValue::Integer(n as u32),
                                    Expr::Ident(s) => OperandValue::StringSlice(s),
                                    Expr::Unary { .. } | Expr::Binary { .. } => {
                                        unimplemented!("expression")
                                    },
                                }
                            },
                        })
                        .collect::<SmallVec<_>>();

                    if !self.context.settings.disable_macro
                        && let Some(mc_instr) = MACRO_INSTRUCTIONS.get(name)
                        && let Some(expanded) = mc_instr
                            .expand(self.context, pc as u32, name, &ops)
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
                            processed.push((instr, LineInfo {
                                original_line: line,
                                label_name: current_label,
                            }));

                            current_label = None;
                        }
                    } else {
                        processed.push(((name, ops), LineInfo {
                            original_line: line,
                            label_name: current_label,
                        }));

                        current_label = None;
                    };
                },
            }
        }

        Ok(processed)
    }
}
