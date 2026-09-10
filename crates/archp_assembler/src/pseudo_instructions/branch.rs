use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Bgt "bgt" |ops| {
        [ Ident(rs1), Ident(rs2), addr @ (Ident(..) | Addition(..)) ] => [
            ("blt", ops![rs2, rs1, addr])
        ];
    }
}

pseudo_instruction! {
    pub Ble "ble" |ops| {
        [ Ident(rs1), Ident(rs2), addr @ (Ident(..) | Addition(..)) ] => [
            ("bge", ops![rs2, rs1, addr])
        ];
    }
}

pseudo_instruction! {
    pub Bgtu "bgtu" |ops| {
        [ Ident(rs1), Ident(rs2), addr @ (Ident(..) | Addition(..)) ] => [
            ("bltu", ops![rs2, rs1, addr])
        ];
    }
}

pseudo_instruction! {
    pub Bleu "bleu" |ops| {
        [ Ident(rs1), Ident(rs2), addr @ (Ident(..) | Addition(..)) ] => [
            ("bgeu", ops![rs2, rs1, addr])
        ];
    }
}

pseudo_instruction! {
    pub Beqz "beqz" |ops| {
        [ Ident(rs), addr @ (Ident(..) | Addition(..)) ] => [
            ("beq", ops![rs, "r0", addr])
        ];
    }
}

pseudo_instruction! {
    pub Bnez "bnez" |ops| {
        [ Ident(rs), addr @ (Ident(..) | Addition(..)) ] => [
            ("bne", ops![rs, "r0", addr])
        ];
    }
}

pseudo_instruction! {
    pub Bltz "bltz" |ops| {
        [ Ident(rs), addr @ (Ident(..) | Addition(..)) ] => [
            ("blt", ops![rs, "r0", addr])
        ];
    }
}

pseudo_instruction! {
    pub Bgez "bgez" |ops| {
        [ Ident(rs), addr @ (Ident(..) | Addition(..)) ] => [
            ("bge", ops![rs, "r0", addr])
        ];
    }
}

pseudo_instruction! {
    pub Blez "blez" |ops| {
        [ Ident(rs), addr @ (Ident(..) | Addition(..)) ] => [
            ("bge", ops!["r0", rs, addr])
        ];
    }
}

pseudo_instruction! {
    pub Bgtz "bgtz" |ops| {
        [ Ident(rs), addr @ (Ident(..) | Addition(..)) ] => [
            ("blt", ops!["r0", rs, addr])
        ];
    }
}
