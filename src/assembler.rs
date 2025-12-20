use std::collections::HashMap;

use anyhow::Result;
use bimap::BiHashMap;

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
    pub labels: BiHashMap<&'a str, usize>,
    pub addr_to_original: Vec<(usize, &'a str)>,
}

impl<'a> Context<'a> {
    pub fn default_with_settings(settings: AssemblerSettings) -> Self {
        Context {
            settings,
            constants: HashMap::new(),
            labels: BiHashMap::new(),
            addr_to_original: Vec::new(),
        }
    }

    #[cfg(test)]
    pub fn test() -> Self {
        Context {
            settings: AssemblerSettings::default(),
            constants: HashMap::from([("FOO", "42"), ("R1", "r1"), ("R0", "r0")]),
            labels: BiHashMap::from_iter([("start", 0), ("loop", 4), ("end", 16)]),
            addr_to_original: Vec::new(),
        }
    }
}

pub type Line<'src> = (&'src str, Option<&'src str>, Vec<OperandValue<'src>>);

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
