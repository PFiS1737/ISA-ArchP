use crate::{
    encoder::immediate::encode_immediate,
    macro_instructions::{ExpandFn, macro_instruction},
    operand::ops,
};

macro_instruction! {
    pub AutoImmAls {
        names: [
            "add", "sub", "mul", "mulh", "mulhu", "mulhsu", "rem", "div",
            "and", "nand", "or", "nor", "xor", "xnor",
            "sll", "srl", "rol", "ror", "sra",
            "seq", "sne", "slt", "sge", "sltu", "sgeu",
        ],
        operand_count: 3,
        expander: F1,
    }
}

const F1: ExpandFn = |ctx, _, name, ops| {
    let inst = match name {
        "add" => "addi",
        "sub" => "subi",
        "mul" => "muli",
        "mulh" => "mulhi",
        "mulhu" => "mulhiu",
        "mulhsu" => "mulhisu",
        "rem" => "remi",
        "div" => "divi",
        "and" => "andi",
        "nand" => "nandi",
        "or" => "ori",
        "nor" => "nori",
        "xor" => "xori",
        "xnor" => "xnori",
        "sll" => "slli",
        "srl" => "srli",
        "rol" => "roli",
        "ror" => "rori",
        "sra" => "srai",
        "seq" => "seqi",
        "sne" => "snei",
        "slt" => "slti",
        "sge" => "sgei",
        "sltu" => "sltiu",
        "sgeu" => "sgeiu",
        _ => unreachable!(),
    };

    if let Ok(imm) = encode_immediate(ctx, &ops[2]) {
        Some(vec![(inst, ops![ops[0], ops[1], imm])])
    } else {
        None
    }
};

macro_instruction! {
    pub AutoImmBranch {
        names: [
            "beq", "bne", "blt", "bge", "bgt", "ble",
            "bltu", "bgeu", "bgtu", "bleu",
        ],
        operand_count: 3,
        expander: F2,
    }
}

const F2: ExpandFn = |ctx, _, name, ops| {
    if let Ok(n) = encode_immediate(ctx, &ops[1]) {
        if n == 0 {
            Some(vec![(name, ops![ops[0], "r0", ops[2]])])
        } else {
            Some(vec![
                ("li", ops!["r31", n]),
                (name, ops![ops[0], "r31", ops[2]]),
            ])
        }
    } else {
        None
    }
};

macro_instruction! {
    pub AutoImmSet {
        names: [ "sgt", "sle", "sgtu", "sleu" ],
        operand_count: 3,
        expander: F3,
    }
}

const F3: ExpandFn = |ctx, _, name, ops| {
    if let Ok(n) = encode_immediate(ctx, &ops[2]) {
        if n == 0 {
            Some(vec![(name, ops![ops[0], ops[1], "r0"])])
        } else {
            Some(vec![
                ("li", ops!["r31", n]),
                (name, ops![ops[0], ops[1], "r31"]),
            ])
        }
    } else {
        None
    }
};

#[cfg(test)]
mod tests {
    use crate::testkit::*;

    #[test]
    fn auto_imm_als() {
        let add = mc_instr("add");

        assert_snapshot!(add(&["r1", "r2", "r3"]), @"");
        assert_snapshot!(add(&["r1", "r2", "0"]), @"addi r1 r2 0");
        assert_snapshot!(add(&["r1", "r2", "0x123"]), @"addi r1 r2 0x123");
        assert_snapshot!(add(&["r1", "r2", "0x1234"]), @"lui r31 1; addi r31 r31 0x234; add r1 r2 r31");
        assert_snapshot!(add(&["r1", "r2", "0x12345678"]), @"lui r31 0x12345; addi r31 r31 0x678; add r1 r2 r31");

        assert_snapshot!(add(&["r1", "r2", "123"]), @"addi r1 r2 123");
        assert_snapshot!(add(&["r1", "r2", "3000"]), @"lui r31 1; addi r31 r31 0xFFFFFBB8; add r1 r2 r31");
        assert_snapshot!(add(&["r1", "r2", "-123"]), @"addi r1 r2 -123");
        assert_snapshot!(add(&["r1", "r2", "-3000"]), @"lui r31 0xFFFFF; addi r31 r31 0x448; add r1 r2 r31");

        assert_snapshot!(add(&["r1", "r2", "0x123"]), @"addi r1 r2 0x123");
        assert_snapshot!(add(&["r1", "r2", "0x1234"]), @"lui r31 1; addi r31 r31 0x234; add r1 r2 r31");
    }

    #[test]
    fn auto_imm_branch() {
        let beq = mc_instr("beq");

        assert_snapshot!(beq(&["r1", "r2", "0"]), @"");
        assert_snapshot!(beq(&["r1", "0", "0"]), @"beq r1 r0 0");
        assert_snapshot!(beq(&["r1", "0x123", "0"]), @"li r31 0x123; beq r1 r31 0");
        assert_snapshot!(beq(&["r1", "0x1234", "0"]), @"lui r31 1; addi r31 r31 0x234; beq r1 r31 0");
        assert_snapshot!(beq(&["r1", "0x12345678", "0"]), @"lui r31 0x12345; addi r31 r31 0x678; beq r1 r31 0");

        assert_snapshot!(beq(&["r1", "123", "0"]), @"li r31 123; beq r1 r31 0");
        assert_snapshot!(beq(&["r1", "3000", "0"]), @"lui r31 1; addi r31 r31 0xFFFFFBB8; beq r1 r31 0");
        assert_snapshot!(beq(&["r1", "-123", "0"]), @"li r31 -123; beq r1 r31 0");
        assert_snapshot!(beq(&["r1", "-3000", "0"]), @"lui r31 0xFFFFF; addi r31 r31 0x448; beq r1 r31 0");

        assert_snapshot!(beq(&["r1", "0x123", "0"]), @"li r31 0x123; beq r1 r31 0");
        assert_snapshot!(beq(&["r1", "0x1234", "0"]), @"lui r31 1; addi r31 r31 0x234; beq r1 r31 0");
    }

    #[test]
    fn auto_imm_set() {
        let slt = mc_instr("slt");
        assert_snapshot!(slt(&["r1", "r2", "0"]), @"slti r1 r2 0");
        assert_snapshot!(slt(&["r1", "r2", "0x1234"]), @"lui r31 1; addi r31 r31 0x234; slt r1 r2 r31");

        let sle = mc_instr("sle");
        assert_snapshot!(sle(&["r1", "r2", "0"]), @"sle r1 r2 r0");
        assert_snapshot!(sle(&["r1", "r2", "0x1234"]), @"lui r31 1; addi r31 r31 0x234; sle r1 r2 r31");
    }
}
