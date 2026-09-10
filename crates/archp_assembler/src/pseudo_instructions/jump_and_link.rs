use smallvec::smallvec;

use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::{ExpandFn, pseudo_instruction},
};

pseudo_instruction! {
    pub J "j" |ops| {
        [ addr @ (Ident(..) | Addition(..)) ] => [
            ("jal", ops!["r0", addr])
        ];
    }
}

// TODO: relax to 'jal'
pseudo_instruction! {
    pub Jump "jump" {
        [ Ident(..) | Addition(..), Ident(..) ] => F;
    }
}

const F: ExpandFn = |ctx, ops| {
    ctx.add_auipc_relocation("jalr", &ops[0]).unwrap();

    Ok(smallvec![
        ("auipc", ops![ops[1], 0]),
        ("jalr", ops!["r0", ops[1], 0])
    ])
};

pseudo_instruction! {
    pub Jal "jal" |ops| {
        [ addr @ (Ident(..) | Addition(..)) ] => [
            ("jal", ops!["ra", addr])
        ];
        [ Ident(rd), addr @ (Ident(..) | Addition(..)) ] => [
            ("jal", ops![rd, addr])
        ];
    }
}

pseudo_instruction! {
    pub Jr "jr" |ops| {
        [ Ident(rs) ] => [
            ("jalr", ops!["r0", rs, 0])
        ];
        [ Ident(rs), Num(imm) ] => [
            ("jalr", ops!["r0", rs, imm])
        ];
    }
}

pseudo_instruction! {
    pub Jalr "jalr" |ops| {
        [ Ident(rs) ] => [
            ("jalr", ops!["ra", rs, 0])
        ];
        [ Ident(rs), Num(imm) ] => [
            ("jalr", ops!["ra", rs, imm])
        ];
        [ Ident(rd), Ident(rs) ] => [
            ("jalr", ops![rd, rs, 0])
        ];
        [ Ident(rd), Ident(rs), Num(imm) ] => [
            ("jalr", ops![rd, rs, imm])
        ];
    }
}
