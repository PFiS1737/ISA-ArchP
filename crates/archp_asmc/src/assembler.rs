use std::collections::HashMap;

use anyhow::Result;

use crate::{operand::OperandValue, pass1::Pass1, pass2::Pass2};

pub struct Assembler {
    settings: AssemblerSettings,
    source_lines: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default)]
pub struct AssemblerSettings {
    pub disable_macro: bool,
}

#[derive(Debug, Clone, Default)]
pub struct Context<'a> {
    pub settings: AssemblerSettings,

    pub constants: HashMap<&'a str, &'a str>,
    pub labels: HashMap<&'a str, usize>,

    pub instr_info: Vec<InstrInfo<'a>>,
}

#[derive(Debug, Clone, Default)]
pub struct InstrInfo<'a> {
    pub original_line: (usize, &'a str),
    pub label_name: Option<&'a str>,
}

impl<'a> Context<'a> {
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
    pub fn new(settings: AssemblerSettings, source_lines: Vec<String>) -> Self {
        Assembler {
            settings,
            source_lines,
        }
    }

    pub fn assemble(&self) -> Result<(Vec<u32>, Vec<String>)> {
        let mut context = Context::default_with_settings(self.settings);

        let mut pass1 = Pass1::new(&mut context);
        let processed = pass1.run(&self.source_lines)?;

        let pass2 = Pass2::new(&mut context);
        pass2.run(processed)
    }
}
