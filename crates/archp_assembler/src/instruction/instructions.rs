use archp_macros::instructions;
use archp_types::{InstructionType, OperandType};

use crate::instruction::Instruction;

instructions! {
    0b0000000: {
        0: R => add;
        1: R => sub;
        2: R => and;
        3: R => or;
        4: I => addi;
        5: I => subi;
        6: I => andi;
        7: I => ori;
    };

    0b0000001: {
        0: R => xor;
        1: R => xnor;
        2: R => nand;
        3: R => nor;
        4: I => xori;
        5: I => xnori;
        6: I => nandi;
        7: I => nori;
    };

    0b0000010: {
        0: R => mul;
        1: R => mulh;
        2: R => mulhu;
        3: R => mulhsu;
        4: I => muli;
        5: I => mulhi;
        6: I => mulhiu;
        7: I => mulhisu;
    };

    0b0000011: {
        0: R => div;
        1: R => rem;
        2: R => divu;
        3: R => remu;
        4: I => divi;
        5: I => remi;
        6: I => diviu;
        7: I => remiu;
    };

    0b0000100: {
        0: R => sll;
        1: R => srl;
        3: R => sra;
        4: R => rol;
        5: R => ror;
    };

    0b0000101: {
        0: I => slli rd, rs, uimm5;
        1: I => srli rd, rs, uimm5;
        3: I => srai rd, rs, uimm5;
        4: I => roli rd, rs, uimm5;
        5: I => rori rd, rs, uimm5;
    };

    0b0000110: {
        0: R => seq;
        1: R => sne;
        2: R => slt;
        3: R => sge;
        4: R => sltu;
        5: R => sgeu;
    };

    0b0000111: {
        0: I => seqi;
        1: I => snei;
        2: I => slti;
        3: I => sgei;
        4: I => sltiu;
        5: I => sgeiu;
    };

    0b0001000: {
        0: I => lw;
        1: I => lh;
        2: I => lhu;
        3: I => lb;
        4: I => lbu;
        5: S => sw;
        6: S => sh;
        7: S => sb;
    };

    0b0001001: {
        0: B => beq;
        1: B => bne;
        2: B => blt;
        3: B => bge;
        4: B => bltu;
        5: B => bgeu;
        7: I => jalr;
    };

    0b0001010: {
        J => jal;
    };

    0b0001011: {
        U => lui;
    };

    0b0001100: {
        U => auipc;
    };

    0b0100000: {
        0: R => ecall _, _, _;
    };

    #[cfg(feature = "stack")]
    0b1111101: {
        J => call _, addr20;
    };

    #[cfg(feature = "stack")]
    0b1111110: {
        0: I => pop rd, _, _;
        1: I => push _, rs, _;
        2: I => ret _, _, _;
        3: I => callr _, rs, imm12;
    };
}
