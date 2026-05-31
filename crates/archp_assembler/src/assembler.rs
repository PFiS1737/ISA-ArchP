use std::collections::HashMap;

use anyhow::Result;

use crate::{operand::OperandValue, pass1::Pass1, pass2::Pass2};

pub struct Assembler {
    settings: AssemblerSettings,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AssemblerSettings {
    pub disable_macro: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Context<'src> {
    pub settings: AssemblerSettings,

    pub constants: HashMap<&'src str, &'src str>,
    pub labels: HashMap<&'src str, usize>,

    pub instr_info: Vec<InstrInfo<'src>>,
}

#[derive(Debug, Clone, Default)]
pub struct InstrInfo<'src> {
    pub original_line: (usize, &'src str),
    pub label_name: Option<&'src str>,
}

impl<'src> Context<'src> {
    pub fn default_with_settings(settings: AssemblerSettings) -> Self {
        Context {
            settings,
            constants: HashMap::new(),
            labels: HashMap::new(),
            instr_info: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Context {
            settings: AssemblerSettings::default(),
            constants: HashMap::from([("FOO", "42"), ("R1", "r1"), ("R0", "r0")]),
            labels: HashMap::from_iter([
                ("start", 0),
                ("loop", 4),
                ("end", 4094),
                ("over", 0x123456),
            ]),
            instr_info: Vec::new(),
        }
    }
}

pub type Line<'src> = (&'src str, Vec<OperandValue<'src>>);

impl Assembler {
    pub fn new(settings: AssemblerSettings) -> Self {
        Assembler { settings }
    }

    pub fn assemble<'src, T: IntoIterator<Item = &'src str>>(
        &self,
        source_lines: T,
    ) -> Result<(Vec<u32>, Vec<String>)> {
        let mut context = Context::default_with_settings(self.settings);

        let mut pass1 = Pass1::new(&mut context);
        let processed = pass1.run(source_lines)?;

        let pass2 = Pass2::new(&mut context);
        pass2.run(processed)
    }
}
