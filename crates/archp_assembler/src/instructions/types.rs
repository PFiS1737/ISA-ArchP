use std::fmt::Display;

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

impl Display for InstrType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InstrType::R => write!(f, "R"),
            InstrType::I => write!(f, "I"),
            InstrType::B => write!(f, "B"),
            InstrType::S => write!(f, "S"),
            InstrType::U => write!(f, "U"),
            InstrType::J => write!(f, "J"),
        }
    }
}

impl InstrType {
    pub fn encode(&self, opcode: u32, funct3: u32, ops: &[u32]) -> u32 {
        match self {
            InstrType::R => self.encode_r(opcode, funct3, ops),
            InstrType::I => self.encode_i(opcode, funct3, ops),
            InstrType::B => self.encode_b(opcode, funct3, ops),
            InstrType::S => self.encode_s(opcode, funct3, ops),
            InstrType::U => self.encode_u(opcode, ops),
            InstrType::J => self.encode_j(opcode, ops),
        }
    }

    pub fn decode(&self, code: u32) -> SmallVec<[u32; 3]> {
        match self {
            InstrType::R => self.decode_r(code),
            InstrType::I => self.decode_i(code),
            InstrType::B => self.decode_b(code),
            InstrType::S => self.decode_s(code),
            InstrType::U => self.decode_u(code),
            InstrType::J => self.decode_j(code),
        }
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

    encode_r(opcode: u32, funct3: u32, ops: &[u32]) [
        ops[0] => rd;
        ops[1] => rs1;
        ops[2] => rs2;
    ]

    decode_r [ rd, rs1, rs2 ]
);

instr_codec!(
    [
        opcode => (25, 7) => _;
        funct3 => (22, 3) => _;
        rd     => (17, 5) => rd;
        rs1    => (12, 5) => rs1;
        imm12  => (0, 12) => imm12;
    ]

    encode_i(opcode: u32, funct3: u32, ops: &[u32]) [
        ops[0] => rd;
        ops[1] => rs1;
        ops[2] => imm12;
    ]

    decode_i [ rd, rs1, imm12 ]
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

    encode_b(opcode: u32, funct3: u32, ops: &[u32]) [
        ops[0] => rs1;
        ops[1] => rs2;
        ops[2] => offset12;
    ]

    decode_b [
        rs1,
        rs2,
        (offset12_hi << 7) | offset12_lo,
    ]
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

    encode_s(opcode: u32, funct3: u32, ops: &[u32]) [
        ops[0] => rs2;
        ops[1] => rs1;
        ops[2] => offset12;
    ]

    decode_s [
        rs2,
        rs1,
        (offset12_hi << 7) | offset12_lo,
    ]
);

instr_codec!(
    [
        opcode            => (25, 7) => _;
        (imm20 >> 17)     => (22, 3) => imm20_hi;
        rd                => (17, 5) => rd;
        (imm20 & 0x1ffff) => (0, 17) => imm20_lo;
    ]

    encode_u(opcode: u32, ops: &[u32]) [
        ops[0] => rd;
        ops[1] => imm20;
    ]

    decode_u [
        rd,
        (imm20_hi << 17) | imm20_lo,
    ]
);

instr_codec!(
    [
        opcode            => (25, 7) => _;
        (imm20 >> 17)     => (22, 3) => imm20_hi;
        rd                => (17, 5) => rd;
        (imm20 & 0x1ffff) => (0, 17) => imm20_lo;
    ]

    encode_j(opcode: u32, ops: &[u32]) [
        ops[0] => rd;
        ops[1] => imm20;
    ]

    decode_j [
        rd,
        (imm20_hi << 17) | imm20_lo,
    ]
);

macro instr_codec {
    (
        [
            $( $enc_var:expr => ($shift:literal, $len:literal) => $dec_var:tt );+ $(;)?
        ]

        $enc_fn:ident( $($enc_arg:ident : $enc_ty:ty),* $(,)? ) [
            $($enc_op:expr => $enc_let:ident);* $(;)?
        ]

        $dec_fn:ident [
            $( $out:expr ),* $(,)?
        ]
    ) => {
        impl InstrType {
            fn $enc_fn(&self, $($enc_arg : $enc_ty),* ) -> u32 {
                $( let $enc_let = $enc_op );* ;
                $( ((($enc_var) as u32) & ((1u32 << $len) - 1)) << $shift )|*
            }

            fn $dec_fn(&self, code: u32) -> SmallVec<[u32; 3]> {
                $( let $dec_var = ((code >> $shift) & ((1u32 << $len) - 1)); )*
                let mut v = SmallVec::new();
                $( v.push($out) );* ;
                v
            }
        }
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    fn roundtrip(instr_type: InstrType, opcode: u32, funct3: u32, ops: &[u32]) {
        let code = instr_type.encode(opcode, funct3, ops);
        let decoded = instr_type.decode(code);

        assert_eq!(decoded.as_slice(), ops);
    }

    #[test]
    fn test_r_type() {
        roundtrip(InstrType::R, 0b0110011, 0b000, &[1, 2, 3]);
        roundtrip(InstrType::R, 0b0110011, 0b111, &[31, 0, 15]);
        roundtrip(InstrType::R, 0x7f, 0x5, &[0, 31, 17]);
    }

    #[test]
    fn test_i_type() {
        roundtrip(InstrType::I, 0b0010011, 0b000, &[1, 2, 123]);
        roundtrip(InstrType::I, 0b0010011, 0b111, &[31, 0, 0xfff]);
        roundtrip(InstrType::I, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_b_type() {
        roundtrip(InstrType::B, 0b1100011, 0b000, &[1, 2, 0x123]);
        roundtrip(InstrType::B, 0b1100011, 0b111, &[31, 0, 0xfff]);
        roundtrip(InstrType::B, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_s_type() {
        roundtrip(InstrType::S, 0b0100011, 0b000, &[3, 2, 0x123]);
        roundtrip(InstrType::S, 0b0100011, 0b010, &[31, 0, 0xfff]);
        roundtrip(InstrType::S, 0x7f, 0x5, &[0, 31, 0x800]);
    }

    #[test]
    fn test_u_type() {
        roundtrip(InstrType::U, 0b0110111, 0, &[1, 0x12345]);
        roundtrip(InstrType::U, 0b0010111, 0, &[31, 0xfffff]);
        roundtrip(InstrType::U, 0x7f, 0, &[0, 0x80000]);
    }

    #[test]
    fn test_j_type() {
        roundtrip(InstrType::J, 0b1101111, 0, &[1, 0x12345]);
        roundtrip(InstrType::J, 0b1101111, 0, &[31, 0xfffff]);
        roundtrip(InstrType::J, 0x7f, 0, &[0, 0x80000]);
    }
}
