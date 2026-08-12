use std::collections::HashMap;

use anyhow::Result;
use smallvec::SmallVec;

use crate::{operand::Operand, pass1::Pass1, pass2::Pass2};

pub struct Assembler {
    settings: AssemblerSettings,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AssemblerSettings {
    pub disable_macro: bool,
}

#[derive(Default)]
pub struct Context<'src> {
    pub settings: AssemblerSettings,

    pub labels: HashMap<&'src str, usize>,

    /// Maps processed lines to the (original line number, original line content)
    pub source_map: Vec<(usize, &'src str)>,

    /// Register aliases, e.g. '.alias foo, s0'
    pub aliases: HashMap<&'src str, &'src str>,

    /// Constant equates, e.g. '.equ FOO, 42'
    pub equates: HashMap<&'src str, i64>,
}

impl<'src> Context<'src> {
    pub fn default_with_settings(settings: AssemblerSettings) -> Self {
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

pub type Instr<'src> = (&'src str, SmallVec<[Operand<'src>; 3]>);

type AssemblerResult<'src> = Result<((Vec<u32>, Vec<Instr<'src>>), Context<'src>)>;

impl Assembler {
    pub fn new(settings: AssemblerSettings) -> Self {
        Assembler { settings }
    }

    pub fn assemble<'src>(&self, source: &'src str) -> AssemblerResult<'src> {
        let mut context = Context::default_with_settings(self.settings);

        let mut pass1 = Pass1::new(&mut context);
        let processed = pass1.run(source)?;

        let pass2 = Pass2::new(&mut context);
        let res = pass2.run(processed)?;

        Ok((res, context))
    }
}
