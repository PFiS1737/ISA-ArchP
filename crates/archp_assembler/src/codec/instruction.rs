use smallvec::SmallVec;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InstrType {
    R,
    I,
    B,
    S,
    U,
    J,
}

pub fn encode_instruction(itype: InstrType, opcode: u32, funct3: u32, ops: &[u32]) -> u32 {
    match itype {
        InstrType::R => encode_r(opcode, funct3, ops),
        InstrType::I => encode_i(opcode, funct3, ops),
        InstrType::B => encode_b(opcode, funct3, ops),
        InstrType::S => encode_s(opcode, funct3, ops),
        InstrType::U => encode_u(opcode, funct3, ops),
        InstrType::J => encode_j(opcode, funct3, ops),
    }
}

pub fn decode_instruction(itype: InstrType, code: u32) -> SmallVec<[u32; 3]> {
    match itype {
        InstrType::R => decode_r(code),
        InstrType::I => decode_i(code),
        InstrType::B => decode_b(code),
        InstrType::S => decode_s(code),
        InstrType::U => decode_u(code),
        InstrType::J => decode_j(code),
    }
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

    fn test(instr_type: InstrType, opcode: u32, funct3: u32, ops: &[u32]) {
        let code = encode_instruction(instr_type, opcode, funct3, ops);
        let decoded = decode_instruction(instr_type, code);

        assert_eq!(decoded.as_slice(), ops);
    }

    #[test]
    fn test_r_type() {
        test(InstrType::R, 0b0110011, 0b000, &[1, 2, 3]);
        test(InstrType::R, 0b0110011, 0b111, &[31, 0, 15]);
        test(InstrType::R, 0x7f, 0x5, &[0, 31, 17]);
    }

    #[test]
    fn test_i_type() {
        test(InstrType::I, 0b0010011, 0b000, &[1, 2, 123]);
        test(InstrType::I, 0b0010011, 0b111, &[31, 0, 0xfff]);
        test(InstrType::I, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_b_type() {
        test(InstrType::B, 0b1100011, 0b000, &[1, 2, 0x123]);
        test(InstrType::B, 0b1100011, 0b111, &[31, 0, 0xfff]);
        test(InstrType::B, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_s_type() {
        test(InstrType::S, 0b0100011, 0b000, &[3, 2, 0x123]);
        test(InstrType::S, 0b0100011, 0b010, &[31, 0, 0xfff]);
        test(InstrType::S, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_u_type() {
        test(InstrType::U, 0b0110111, 0, &[1, 0x12345]);
        test(InstrType::U, 0b0010111, 0, &[31, 0xfffff]);
        test(InstrType::U, 0x7f, 0, &[0, 0x80000]);
    }

    #[test]
    fn test_j_type() {
        test(InstrType::J, 0b1101111, 0, &[1, 0x12345]);
        test(InstrType::J, 0b1101111, 0, &[31, 0xfffff]);
        test(InstrType::J, 0x7f, 0, &[0, 0x80000]);
    }
}
