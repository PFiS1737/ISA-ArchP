use crate::{
    operand::{Operand::*, ops},
    pseudo_instructions::pseudo_instruction,
};

pseudo_instruction! {
    pub Sgt "sgt" |ops| {
        [ Ident(rd), Ident(rs1), Ident(rs2) ] => [
            ("slt", ops![rd, rs2, rs1])
        ];
    }
}

pseudo_instruction! {
    pub Sle "sle" |ops| {
        [ Ident(rd), Ident(rs1), Ident(rs2) ] => [
            ("sge", ops![rd, rs2, rs1])
        ];
    }
}

pseudo_instruction! {
    pub Sgtu "sgtu" |ops| {
        [ Ident(rd), Ident(rs1), Ident(rs2) ] => [
            ("sltu", ops![rd, rs2, rs1])
        ];
    }
}

pseudo_instruction! {
    pub Sleu "sleu" |ops| {
        [ Ident(rd), Ident(rs1), Ident(rs2) ] => [
            ("sgeu", ops![rd, rs2, rs1])
        ];
    }
}

pseudo_instruction! {
    pub Seqz "seqz" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("seq", ops![rd, rs, "r0"])
        ];
    }
}

pseudo_instruction! {
    pub Snez "snez" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("sne", ops![rd, rs, "r0"])
        ];
    }
}

pseudo_instruction! {
    pub Sltz "sltz" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("slt", ops![rd, rs, "r0"])
        ];
    }
}

pseudo_instruction! {
    pub Sgez "sgez" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("sge", ops![rd, rs, "r0"])
        ];
    }
}

pseudo_instruction! {
    pub Slez "slez" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("sge", ops![rd, "r0", rs])
        ];
    }
}

pseudo_instruction! {
    pub Sgtz "sgtz" |ops| {
        [ Ident(rd), Ident(rs) ] => [
            ("slt", ops![rd, "r0", rs])
        ];
    }
}
