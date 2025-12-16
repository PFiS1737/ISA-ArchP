use std::collections::HashMap;

use anyhow::Result;
use bimap::BiHashMap;

use crate::{operand::OperandValue, pass1::Pass1, pass2::Pass2};

pub struct Assembler {
    settings: AssemblerSettings,
    source_lines: Vec<String>,
}

#[derive(Debug, Clone, Copy)]
pub struct AssemblerSettings {
    pub disable_macro: bool,
}

#[derive(Debug, Clone)]
pub struct Context<'a> {
    pub settings: AssemblerSettings,
    pub constants: HashMap<&'a str, &'a str>,
    pub labels: BiHashMap<&'a str, usize>,
    pub addr_to_original: Vec<(usize, &'a str)>,
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
        let mut context = Context {
            settings: self.settings,
            constants: HashMap::new(),
            labels: BiHashMap::new(),
            addr_to_original: Vec::new(),
        };

        let mut pass1 = Pass1::new(&mut context);
        let processed = pass1.run(&self.source_lines)?;

        let pass2 = Pass2::new(&mut context);
        pass2.run(processed)
    }
}
