mod als_imm32;
mod branch_imm;
mod load_imm32;

use std::collections::{HashMap, VecDeque};

use anyhow::{Result, bail};
use once_cell::sync::Lazy;

use crate::operand::OperandValue;

type ExpandRet<'a> = Result<Option<Vec<(&'static str, Option<&'a str>, Vec<OperandValue<'a>>)>>>;
type ExpandFn =
    for<'a> fn(&'static str, &'static str, Option<&'a str>, &[OperandValue<'a>]) -> ExpandRet<'a>;

#[derive(Debug, Clone, Copy)]
pub struct MacroInstruction {
    name: &'static str,
    operand_count: usize,
    expander: ExpandFn,
}

inventory::collect!(MacroInstruction);

pub static MACRO_INSTRUCTIONS: Lazy<HashMap<&'static str, MacroInstruction>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for entry in inventory::iter::<MacroInstruction> {
        map.insert(entry.name, *entry);
    }
    map
});

impl MacroInstruction {
    pub fn expand<'a>(
        &self,
        cond: Option<&'a str>,
        operands: &[OperandValue<'a>],
    ) -> ExpandRet<'a> {
        self.assert_operand_count(operands)?;

        let mut deq: VecDeque<_> = match (self.expander)(self.name, self.name, cond, operands)? {
            None => return Ok(None),
            Some(v) => v.into(),
        };

        let mut ret = Vec::new();

        while let Some((name, cond, ops)) = deq.pop_front() {
            if let Some(mc) = MACRO_INSTRUCTIONS.get(name) {
                mc.assert_operand_count(&ops)?;

                match (mc.expander)(self.name, name, cond, &ops)? {
                    None => {
                        ret.push((name, cond, ops));
                    }
                    Some(v) => {
                        let mut q: VecDeque<_> = v.into();
                        q.append(&mut deq);
                        deq = q;
                    }
                }
            } else {
                ret.push((name, cond, ops));
            }
        }

        Ok(Some(ret))
    }

    fn assert_operand_count(&self, operands: &[OperandValue]) -> Result<()> {
        if operands.len() != self.operand_count {
            bail!(
                "Macro-instruction '{}' requires {} operands, got {}",
                self.name,
                self.operand_count,
                operands.len()
            );
        }

        Ok(())
    }
}

macro macro_instruction {
    (
        name: [ $($name:literal),+ $(,)? ],
        operand_count: $count:literal,
        expander: $expander:expr,
    ) => {
        $(
            $crate::macro_instructions::macro_instruction! {
                name: $name,
                operand_count: $count,
                expander: $expander,
            }
        )+
    },

    (
        name: $name:literal,
        operand_count: $count:literal,
        expander: $expander:expr,
    ) => {
        inventory::submit! {
            $crate::macro_instructions::MacroInstruction {
                name: $name,
                operand_count: $count,
                expander: $expander,
            }
        }
    },
}
