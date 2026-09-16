use anyhow::Result;
use smallvec::SmallVec;

use crate::{
    context::Context,
    operand::{DirectiveOperand, Operand},
    pass1::Pass1,
    pass2::Pass2,
};

pub type Instr<'src> = (&'src str, SmallVec<[Operand<'src>; 3]>);

#[derive(Debug)]
pub enum Line<'src> {
    Label(&'src str),
    Directive {
        name: &'src str,
        operands: Vec<DirectiveOperand<'src>>,
        line: (usize, &'src str),
    },
    Instruction {
        name: &'src str,
        operands: SmallVec<[Operand<'src>; 3]>,
        line: (usize, &'src str),
    },
}

pub struct Assembler;

impl Assembler {
    pub fn assemble<'src>(source: &'src str) -> Result<Context<'src>> {
        let mut context = Context::default();

        let mut pass1 = Pass1::new(&mut context);
        pass1.run(source)?;

        let mut pass2 = Pass2::new(&mut context);
        pass2.run()?;

        Ok(context)
    }
}
