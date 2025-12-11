use anyhow::Result;

use crate::{pass1::Pass1, pass2::Pass2};

pub struct Assembler {
    settings: AssemblerSettings,
    source_lines: Vec<String>,
}

pub struct AssemblerSettings {
    pub disable_macro: bool,
}

impl Assembler {
    pub fn new(settings: AssemblerSettings, source_lines: Vec<String>) -> Self {
        Assembler {
            settings,
            source_lines,
        }
    }

    pub fn assemble(&self) -> Result<(Vec<u32>, Vec<String>)> {
        let mut pass1 = Pass1::new(self.settings.disable_macro);
        pass1.run(&self.source_lines)?;

        let pass2 = Pass2::new(pass1.labels, pass1.addr_to_original);
        pass2.run(pass1.processed)
    }
}
