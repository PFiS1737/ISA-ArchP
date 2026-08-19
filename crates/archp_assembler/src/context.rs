use std::collections::HashMap;

use anyhow::{Result, bail};

use crate::{AssemblerSettings, assembler::Instr, instructions::Entry, operand::Operand};

#[derive(Default)]
pub struct Context<'src> {
    pub settings: AssemblerSettings,

    /// The generated machine code (raw bytes, little-endian)
    pub text: Vec<u8>,

    /// The processed instructions, after macro and pseudo-instruction expansion
    pub instrs: HashMap<usize, Option<Instr<'src>>>,

    pub labels: HashMap<&'src str, usize>,

    /// Register aliases, e.g. '.alias foo, s0'
    pub aliases: HashMap<&'src str, &'src str>,

    /// Constant equates, e.g. '.equ FOO, 42'
    pub equates: HashMap<&'src str, i64>,

    pub relocations: Vec<Relocation<'src>>,
}

#[derive(Debug, Clone)]
pub struct Relocation<'a> {
    pub offset: usize,
    pub label: &'a str,
    pub addend: i64,
    pub instr: &'static Entry,
}

impl<'src> Context<'src> {
    pub fn add_byte(&mut self, byte: u8) {
        self.text.push(byte);
    }

    pub fn add_code(&mut self, code: u32, instr: Option<Instr<'src>>) {
        let offset = self.text.len();

        let bytes = code.to_le_bytes();
        self.text.extend_from_slice(&bytes);

        self.instrs.insert(offset, instr);
    }

    pub fn get_code(&self, offset: usize) -> u32 {
        let end = offset + 4;
        assert!(end <= self.text.len(), "get_code out of bounds");

        let bytes: [u8; 4] = self.text[offset..end].try_into().unwrap();

        u32::from_le_bytes(bytes)
    }

    pub fn set_code(&mut self, offset: usize, value: u32) {
        let end = offset + 4;
        assert!(end <= self.text.len(), "set_code out of bounds");

        let bytes = value.to_le_bytes();
        self.text[offset..end].copy_from_slice(&bytes);
    }
}

impl Context<'_> {
    pub fn finish(&mut self) {
        self.align4();
    }

    fn align4(&mut self) {
        let rem = self.text.len() & 3;
        if rem != 0 {
            let padding = 4 - rem;
            self.text.reserve(padding);
            self.text.extend_from_slice(&[0u8; 4][..padding]);
        }
    }
}

impl<'src> Context<'src> {
    pub fn add_relocation(
        &mut self,
        instr: &'static Entry,
        offset: usize,
        op: &Operand<'src>,
    ) -> Result<()> {
        let (label, addend) = match op {
            Operand::Ident(s) => (s, 0),
            Operand::Addition(s, n) => (s, *n),
            _ => bail!("Expected address label, got: {}", op),
        };

        self.relocations.push(Relocation {
            offset,
            label,
            addend,
            instr,
        });

        Ok(())
    }
}

impl<'src> Context<'src> {
    pub fn new(settings: AssemblerSettings) -> Self {
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
