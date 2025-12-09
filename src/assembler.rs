use anyhow::Result;

use crate::{pass1::Pass1, pass2::Pass2};

pub struct Assembler {
    source_lines: Vec<String>,
}

impl Assembler {
    pub fn new(source_lines: Vec<String>) -> Self {
        Assembler { source_lines }
    }

    pub fn assemble(&self) -> Result<(Vec<u32>, Vec<String>)> {
        let mut pass1 = Pass1::new();
        pass1.run(&self.source_lines)?;

        let pass2 = Pass2::new(pass1.labels, pass1.addr_to_original);
        pass2.run(
            pass1
                .processed
                .iter()
                .map(|e| e.iter().map(|s| s.as_str()).collect())
                .collect(),
        )
    }
}
