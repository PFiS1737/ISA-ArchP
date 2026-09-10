use anyhow::Result;
use smallvec::{SmallVec, smallvec};

use crate::{
    assembler::Instr,
    context::Context,
    operand::{
        Operand::{self, *},
        ops,
    },
    pseudo_instructions::pseudo_instruction,
};

fn make_default(
    name: &'static str,
) -> impl for<'a> Fn(&mut Context<'a>, &[Operand<'a>]) -> Result<SmallVec<[Instr<'a>; 2]>> {
    move |_, ops| Ok(smallvec![(name, ops![ops[0], ops[1], ops[2]])])
}

pseudo_instruction! {
    pub Lw "lw" {
        [ Ident(..), Ident(..) | Addition(..) ] => make_f1("lw");
        [ Ident(..), Ident(..), Num(..) ] => make_default("lw");
    }
}

pseudo_instruction! {
    pub Lh "lh" {
        [ Ident(..), Ident(..) | Addition(..) ] => make_f1("lh");
        [ Ident(..), Ident(..), Num(..) ] => make_default("lh");
    }
}

pseudo_instruction! {
    pub Lhu "lhu" {
        [ Ident(..), Ident(..) | Addition(..) ] => make_f1("lhu");
        [ Ident(..), Ident(..), Num(..) ] => make_default("lhu");
    }
}

pseudo_instruction! {
    pub Lb "lb" {
        [ Ident(..), Ident(..) | Addition(..) ] => make_f1("lb");
        [ Ident(..), Ident(..), Num(..) ] => make_default("lb");
    }
}

pseudo_instruction! {
    pub Lbu "lbu" {
        [ Ident(..), Ident(..) | Addition(..) ] => make_f1("lbu");
        [ Ident(..), Ident(..), Num(..) ] => make_default("lbu");
    }
}

fn make_f1(
    name: &'static str,
) -> impl for<'a> Fn(&mut Context<'a>, &[Operand<'a>]) -> Result<SmallVec<[Instr<'a>; 2]>> {
    move |ctx, ops| {
        ctx.add_auipc_relocation(name, &ops[1]).unwrap();

        Ok(smallvec![
            ("auipc", ops![ops[0], 0]),
            (name, ops![ops[0], ops[0], 0])
        ])
    }
}

pseudo_instruction! {
    pub Sw "sw" {
        [ Ident(..), Ident(..) | Addition(..), Ident(..) ] => make_f2("sw");
        [ Ident(..), Ident(..), Num(..) ] => make_default("sw");
    }
}

pseudo_instruction! {
    pub Sh "sh" {
        [ Ident(..), Ident(..) | Addition(..), Ident(..) ] => make_f2("sh");
        [ Ident(..), Ident(..), Num(..) ] => make_default("sh");
    }
}

pseudo_instruction! {
    pub Sb "sb" {
        [ Ident(..), Ident(..) | Addition(..), Ident(..) ] => make_f2("sb");
        [ Ident(..), Ident(..), Num(..) ] => make_default("sb");
    }
}

fn make_f2(
    name: &'static str,
) -> impl for<'a> Fn(&mut Context<'a>, &[Operand<'a>]) -> Result<SmallVec<[Instr<'a>; 2]>> {
    move |ctx, ops| {
        ctx.add_auipc_relocation(name, &ops[1]).unwrap();

        Ok(smallvec![
            ("auipc", ops![ops[2], 0]),
            (name, ops![ops[0], ops[2], 0])
        ])
    }
}
