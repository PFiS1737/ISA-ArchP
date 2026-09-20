use anyhow::{Result, bail};

codec! {
    encode_register, decode_register;

    reg !=> bail!("Invalid register: {}", reg);

    0  <=> "zero" | "r0",
    1  <=> "ra"   | "r1",
    2  <=> "sp"   | "r2",
    3  <=> "gp"   | "r3",
    4  <=> "tp"   | "r4",
    5  <=> "t0"   | "r5",
    6  <=> "t1"   | "r6",
    7  <=> "t2"   | "r7",
    8  <=> "s0"   | "r8" | "fp",
    9  <=> "s1"   | "r9",
    10 <=> "a0"   | "r10",
    11 <=> "a1"   | "r11",
    12 <=> "a2"   | "r12",
    13 <=> "a3"   | "r13",
    14 <=> "a4"   | "r14",
    15 <=> "a5"   | "r15",
    16 <=> "a6"   | "r16",
    17 <=> "a7"   | "r17",
    18 <=> "s2"   | "r18",
    19 <=> "s3"   | "r19",
    20 <=> "s4"   | "r20",
    21 <=> "s5"   | "r21",
    22 <=> "s6"   | "r22",
    23 <=> "s7"   | "r23",
    24 <=> "s8"   | "r24",
    25 <=> "s9"   | "r25",
    26 <=> "s10"  | "r26",
    27 <=> "s11"  | "r27",
    28 <=> "t3"   | "r28",
    29 <=> "t4"   | "r29",
    30 <=> "t5"   | "r30",
    31 <=> "t6"   | "r31",
}

macro codec {
    (
        $fn_encode:ident, $fn_decode:ident;
        $reg:ident !=> $err:expr;
        $( $id:literal <=> $name:literal | $( $others:literal )|+ ),+ ,
    ) => {
        pub fn $fn_encode($reg: &str) -> Result<u32> {
            Ok(match $reg {
                $( $name | $( $others )|+ => $id, )+
                _ => $err,
            })
        }

        pub fn $fn_decode($reg: u32) -> &'static str {
            match $reg {
                $( $id => $name, )+
                _ => unreachable!(),
            }
        }
    }
}
