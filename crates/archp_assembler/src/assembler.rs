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

#[derive(Debug)]
pub enum Line<'src> {
    Label(&'src str),
    Instr {
        name: &'src str,
        operands: SmallVec<[Operand<'src>; 3]>,
        line: (usize, &'src str),
    },
}

impl Assembler {
    pub fn new(settings: AssemblerSettings) -> Self {
        Assembler { settings }
    }

    pub fn assemble<'src>(&self, source: &'src str) -> Result<Context<'src>> {
        let mut context = Context::new(self.settings);

        let mut pass1 = Pass1::new(&mut context);
        pass1.run(source)?;

        let mut pass2 = Pass2::new(&mut context);
        pass2.run()?;

        Ok(context)
    }
}
