use anyhow::{Result, bail};
use smallvec::SmallVec;

use crate::{
    codec::{
        immediate::encode_immediate,
        register::{decode_register, encode_register},
    },
    context::Context,
    operand::Operand,
    relocation::RelocationType,
    types::OperandType,
    utils::sig_ext::sign_extend,
};

pub fn encode_operands<'src>(
    ctx: &mut Context<'src>,
    name: &'static str,
    format: &'static [OperandType],
    operands: &[Operand<'src>],
) -> Result<SmallVec<[u32; 3]>> {
    // TODO: make this static
    let expected = format
        .iter()
        .filter(|x| !matches!(x, OperandType::None))
        .count();

    let count = operands.len();
    if count != expected {
        bail!(
            "Instruction '{}' requires {} operands, got {}",
            name,
            expected,
            count
        );
    }

    let mut ops = operands.iter();

    let mut ret = SmallVec::new();

    for op_ty in format {
        let val = match *op_ty {
            OperandType::RegD | OperandType::RegS => {
                let s = ops.next().unwrap().cast_register()?;
                let reg = ctx.aliases.get(s).unwrap_or(&s);
                encode_register(reg)?
            },
            OperandType::Imm(bits, signed) => {
                let n = ops.next().unwrap().cast_immediate()?;
                encode_immediate(n, bits, signed)?
            },
            OperandType::Addr(bits) => {
                let offset = ctx.text.len();
                ctx.add_relocation(
                    name,
                    RelocationType::Bits(bits),
                    offset,
                    offset,
                    ops.next().unwrap(),
                )?;
                0
            },
            OperandType::None => 0,
        };

        ret.push(val);
    }

    Ok(ret)
}

pub fn decode_operands(
    format: &'static [OperandType],
    operands: SmallVec<[u32; 3]>,
) -> SmallVec<[Operand<'static>; 3]> {
    assert_eq!(format.len(), operands.len());

    let mut ret = SmallVec::new();

    for (op_ty, val) in format.iter().zip(operands) {
        let op = match *op_ty {
            OperandType::RegD | OperandType::RegS => Operand::Ident(decode_register(val)),
            OperandType::Imm(bits, signed) => {
                if signed {
                    Operand::Num(sign_extend(val, bits) as i32 as i64)
                } else {
                    Operand::Num(val as i64)
                }
            },
            OperandType::Addr(bits) => Operand::Num(sign_extend(val, bits) as i32 as i64),
            OperandType::None => continue,
        };

        ret.push(op);
    }

    ret
}
