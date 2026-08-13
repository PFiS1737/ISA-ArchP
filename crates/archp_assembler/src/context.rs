use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::{AssemblerSettings, assembler::Instr, instructions::Entry, operand::Operand};

#[derive(Default)]
pub struct Context<'src> {
    pub settings: AssemblerSettings,

    /// The generated machine code
    pub codes: Vec<u32>,
    /// The processed instructions, after macro and pseudo-instruction expansion
    pub instrs: Vec<Instr<'src>>,
    /// Maps processed instructions to the (original line number, original line content)
    pub source_map: Vec<(usize, &'src str)>,

    pub labels: HashMap<&'src str, usize>,

    /// Register aliases, e.g. '.alias foo, s0'
    pub aliases: HashMap<&'src str, &'src str>,

    /// Constant equates, e.g. '.equ FOO, 42'
    pub equates: HashMap<&'src str, i64>,

    pub relocations: Vec<Relocation<'src>>,
}

pub struct Relocation<'a> {
    pub offset: usize,
    pub label: &'a str,
    pub addend: i64,
    pub instr: &'static Entry,
}

impl<'src> Context<'src> {
    pub fn add_relocation(
        &mut self,
        instr: &'static Entry,
        pc: usize,
        op: &Operand<'src>,
    ) -> Result<()> {
        let (label, addend) = match op {
            Operand::Ident(s) => (s, 0),
            Operand::Addition(s, n) => (s, *n),
            _ => bail!("Expected address label, got: {}", op),
        };

        self.relocations.push(Relocation {
            offset: pc,
            label,
            addend,
            instr,
        });

        Ok(())
    }
}

impl<'src> Context<'src> {
    pub fn new(settings: AssemblerSettings) -> Self {
        Self {
            settings,
            ..Default::default()
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Self {
            labels: HashMap::from_iter([
                ("start", 0),
                ("loop", 4),
                ("end", 4094),
                ("over", 0x123456),
            ]),
            aliases: HashMap::from_iter([("R1", "r1"), ("R0", "r0")]),
            equates: HashMap::from_iter([("FOO", 42), ("BAR", 0x123456)]),
            ..Default::default()
        }
    }
}
