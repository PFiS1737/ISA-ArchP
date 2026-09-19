use smallvec::SmallVec;

use crate::types::InstructionType;

pub fn encode_instruction(itype: InstructionType, opcode: u32, funct3: u32, ops: &[u32]) -> u32 {
    match itype {
        InstructionType::R => encode_r(opcode, funct3, ops),
        InstructionType::I => encode_i(opcode, funct3, ops),
        InstructionType::B => encode_b(opcode, funct3, ops),
        InstructionType::S => encode_s(opcode, funct3, ops),
        InstructionType::U => encode_u(opcode, funct3, ops),
        InstructionType::J => encode_j(opcode, funct3, ops),
    }
}

pub fn decode_instruction(itype: InstructionType, code: u32) -> SmallVec<[u32; 3]> {
    match itype {
        InstructionType::R => decode_r(code),
        InstructionType::I => decode_i(code),
        InstructionType::B => decode_b(code),
        InstructionType::S => decode_s(code),
        InstructionType::U => decode_u(code),
        InstructionType::J => decode_j(code),
    }
}

pub fn decode_opcode(code: u32) -> (u32, u32) {
    let opcode = code >> 25;
    let funct3 = (code >> 22) & 0b111;
    (opcode, funct3)
}

instr_codec!(
    [
        opcode => (25, 7) => _;
        funct3 => (22, 3) => _;
        rd     => (17, 5) => rd;
        rs1    => (12, 5) => rs1;
        0      => (5 , 7) => _;
        rs2    => (0 , 5) => rs2;
    ]

    encode_r(opcode, funct3) [ rd, rs1, rs2 ]

    decode_r() [ rd, rs1, rs2 ]
);

instr_codec!(
    [
        opcode => (25, 7) => _;
        funct3 => (22, 3) => _;
        rd     => (17, 5) => rd;
        rs1    => (12, 5) => rs1;
        imm12  => (0, 12) => imm12;
    ]

    encode_i(opcode, funct3) [ rd, rs1, imm12 ]

    decode_i() [ rd, rs1, imm12 ]
);

instr_codec!(
    [
        opcode            => (25, 7) => _;
        funct3            => (22, 3) => _;
        (offset12 >> 7)   => (17, 5) => offset12_hi;
        rs1               => (12, 5) => rs1;
        (offset12 & 0x7f) => (5 , 7) => offset12_lo;
        rs2               => (0 , 5) => rs2;
    ]

    encode_b(opcode, funct3) [ rs1, rs2, offset12 ]

    decode_b() [ rs1, rs2, (offset12_hi << 7) | offset12_lo ]
);

instr_codec!(
    [
        opcode            => (25, 7) => _;
        funct3            => (22, 3) => _;
        (offset12 >> 7)   => (17, 5) => offset12_hi;
        rs1               => (12, 5) => rs1;
        (offset12 & 0x7f) => (5 , 7) => offset12_lo;
        rs2               => (0 , 5) => rs2;
    ]

    encode_s(opcode, funct3) [ rs2, rs1, offset12 ]

    decode_s() [ rs2, rs1, (offset12_hi << 7) | offset12_lo ]
);

instr_codec!(
    [
        opcode            => (25, 7) => _;
        (imm20 >> 17)     => (22, 3) => imm20_hi;
        rd                => (17, 5) => rd;
        (imm20 & 0x1ffff) => (0, 17) => imm20_lo;
    ]

    encode_u(opcode, _) [ rd, imm20 ]

    decode_u() [ rd, (imm20_hi << 17) | imm20_lo ]
);

instr_codec!(
    [
        opcode            => (25, 7) => _;
        (imm20 >> 17)     => (22, 3) => imm20_hi;
        rd                => (17, 5) => rd;
        (imm20 & 0x1ffff) => (0, 17) => imm20_lo;
    ]

    encode_j(opcode, _) [ rd, imm20 ]

    decode_j() [ rd, (imm20_hi << 17) | imm20_lo ]
);

macro instr_codec {
    (
        [
            $( $enc_var:expr => ($shift:literal, $len:literal) => $dec_var:tt );+ $(;)?
        ]

        $enc_fn:ident ( $opcode:ident, $funct3:tt ) [
            $( $in:ident ),* $(,)?
        ]

        $dec_fn:ident () [
            $( $out:expr ),* $(,)?
        ]
    ) => {
        fn $enc_fn($opcode: u32, $funct3: u32, ops: &[u32]) -> u32 {
            let [ $($in),* ] = *ops else { unreachable!(); };
            $( ((($enc_var) as u32) & ((1u32 << $len) - 1)) << $shift )|*
        }

        fn $dec_fn(code: u32) -> SmallVec<[u32; 3]> {
            $( let $dec_var = ((code >> $shift) & ((1u32 << $len) - 1)); )*
            let mut v = SmallVec::new();
            $( v.push($out) );* ;
            v
        }
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test(instr_type: InstructionType, opcode: u32, funct3: u32, ops: &[u32]) {
        let code = encode_instruction(instr_type, opcode, funct3, ops);
        let decoded = decode_instruction(instr_type, code);

        assert_eq!(decoded.as_slice(), ops);
    }

    #[test]
    fn test_r_type() {
        test(InstructionType::R, 0b0110011, 0b000, &[1, 2, 3]);
        test(InstructionType::R, 0b0110011, 0b111, &[31, 0, 15]);
        test(InstructionType::R, 0x7f, 0x5, &[0, 31, 17]);
    }

    #[test]
    fn test_i_type() {
        test(InstructionType::I, 0b0010011, 0b000, &[1, 2, 123]);
        test(InstructionType::I, 0b0010011, 0b111, &[31, 0, 0xfff]);
        test(InstructionType::I, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_b_type() {
        test(InstructionType::B, 0b1100011, 0b000, &[1, 2, 0x123]);
        test(InstructionType::B, 0b1100011, 0b111, &[31, 0, 0xfff]);
        test(InstructionType::B, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_s_type() {
        test(InstructionType::S, 0b0100011, 0b000, &[3, 2, 0x123]);
        test(InstructionType::S, 0b0100011, 0b010, &[31, 0, 0xfff]);
        test(InstructionType::S, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_u_type() {
        test(InstructionType::U, 0b0110111, 0, &[1, 0x12345]);
        test(InstructionType::U, 0b0010111, 0, &[31, 0xfffff]);
        test(InstructionType::U, 0x7f, 0, &[0, 0x80000]);
    }

    #[test]
    fn test_j_type() {
        test(InstructionType::J, 0b1101111, 0, &[1, 0x12345]);
        test(InstructionType::J, 0b1101111, 0, &[31, 0xfffff]);
        test(InstructionType::J, 0x7f, 0, &[0, 0x80000]);
    }
}
