use anyhow::Result;
use smallvec::SmallVec;

use crate::{context::Context, operand::Operand, pass1::Pass1, pass2::Pass2};

pub struct Assembler {
    settings: AssemblerSettings,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AssemblerSettings {
    pub disable_macro: bool,
}

pub type Instr<'src> = (&'src str, SmallVec<[Operand<'src>; 3]>);

type AssemblerResult<'src> = Result<((Vec<u32>, Vec<Instr<'src>>), Context<'src>)>;

impl Assembler {
    pub fn new(settings: AssemblerSettings) -> Self {
        Assembler { settings }
    }

    pub fn assemble<'src>(&self, source: &'src str) -> AssemblerResult<'src> {
        let mut context = Context::new(self.settings);

        let mut pass1 = Pass1::new(&mut context);
        let processed = pass1.run(source)?;

        let pass2 = Pass2::new(&mut context);
        let res = pass2.run(processed)?;

        Ok((res, context))
    }
}
