use std::collections::HashMap;

use crate::AssemblerSettings;

#[derive(Default)]
pub struct Context<'src> {
    pub settings: AssemblerSettings,

    pub labels: HashMap<&'src str, usize>,

    /// Maps processed instructions to the (original line number, original line content)
    pub source_map: Vec<(usize, &'src str)>,

    /// Register aliases, e.g. '.alias foo, s0'
    pub aliases: HashMap<&'src str, &'src str>,

    /// Constant equates, e.g. '.equ FOO, 42'
    pub equates: HashMap<&'src str, i64>,
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
