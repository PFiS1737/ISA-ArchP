use anyhow::{anyhow, bail};
use nom::{
    Parser,
    character::complete::{char, space1},
    combinator::opt,
    sequence::{preceded, terminated},
};
use smallvec::SmallVec;

use crate::{
    assembler::Context,
    operand::Operand,
    parser::{Result, ident, operand::operand, types::line::Line, ws},
};

fn label(input: &str) -> Result<'_, &str> {
    terminated(ident, ws(char(':'))).parse(input)
}

fn operands<'ctx, 'src: 'ctx>(
    ctx: &'ctx Context<'src>,
    input: &'src str,
) -> Result<'src, SmallVec<[Operand<'src>; 3]>> {
    let mut out = SmallVec::new();

    let (input, _) = operand(ctx, input, &mut out)?;

    let (input, Some(_)) = opt(ws(char(','))).parse(input)? else {
        return Ok((input, out));
    };

    let (input, _) = operand(ctx, input, &mut out)?;

    let (input, Some(_)) = opt(ws(char(','))).parse(input)? else {
        return Ok((input, out));
    };

    let (input, _) = operand(ctx, input, &mut out)?;

    Ok((input, out))
}

fn instr<'ctx, 'src: 'ctx>(
    ctx: &'ctx Context<'src>,
    line_no: usize,
    input: &'src str,
) -> Result<'src, Line<'src>> {
    let raw = input;

    let (input, name) = ident(input)?;

    let (input, ops) = opt(preceded(space1, |input| operands(ctx, input))).parse(input)?;

    Ok((input, Line::Instr {
        name,
        operands: ops.unwrap_or_default(),
        line: (line_no, raw),
    }))
}

fn strip_comment(input: &str) -> &str {
    let mut end = input.len();
    for (i, c) in input.char_indices() {
        if c == '#' || c == ';' {
            end = i;
            break;
        }
    }
    &input[..end]
}

pub fn parse_line<'ctx, 'src: 'ctx>(
    ctx: &'ctx Context<'src>,
    line_num: usize,
    line: &'src str,
) -> anyhow::Result<SmallVec<[Line<'src>; 2]>> {
    let line = strip_comment(line).trim();

    if line.is_empty() {
        return Ok(SmallVec::new());
    }

    let mut out = SmallVec::new();
    let mut rest = line;

    while let Ok((r, l)) = label(rest) {
        out.push(Line::Label(l));
        rest = r;
    }

    if rest.is_empty() {
        return Ok(out);
    }

    let (remain, instr) = instr(ctx, line_num, rest)
        .map_err(|e| anyhow!("Error parsing line {}: '{}': {}", line_num, line, e))?;

    if !remain.is_empty() {
        bail!("Unexpected content after line {}: '{}'", line_num, remain);
    }

    out.push(instr);

    Ok(out)
}

#[cfg(test)]
mod tests {
    use insta::assert_debug_snapshot;

    use super::*;

    fn parse_source(input: &str) -> anyhow::Result<Vec<Line<'_>>> {
        let mut lines = Vec::new();

        for (line_no, line) in input.lines().enumerate() {
            let parsed = parse_line(&Context::default(), line_no + 1, line)?;

            lines.extend(parsed);
        }

        Ok(lines)
    }

    fn parse_ok(input: &str) -> Vec<Line<'_>> {
        parse_source(input).expect("parse failed")
    }

    #[test]
    fn empty_and_comment() {
        assert_debug_snapshot!(parse_ok("# comment \n; comment"), @"[]"
        );
    }

    #[test]
    fn instruction_without_operand() {
        assert_debug_snapshot!(parse_ok("ecall"), @r#"
        [
            Instr {
                name: "ecall",
                operands: [],
                line: (
                    1,
                    "ecall",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn instruction_with_basic_operands() {
        assert_debug_snapshot!(parse_ok("j .L1"), @r#"
        [
            Instr {
                name: "j",
                operands: [
                    Ident(
                        ".L1",
                    ),
                ],
                line: (
                    1,
                    "j .L1",
                ),
            },
        ]
        "#
        );
        assert_debug_snapshot!(parse_ok("la x1, hello"), @r#"
        [
            Instr {
                name: "la",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Ident(
                        "hello",
                    ),
                ],
                line: (
                    1,
                    "la x1, hello",
                ),
            },
        ]
        "#
        );
        assert_debug_snapshot!(parse_ok("li x1, 123"), @r#"
        [
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Num(
                        123,
                    ),
                ],
                line: (
                    1,
                    "li x1, 123",
                ),
            },
        ]
        "#
        );
        assert_debug_snapshot!(parse_ok("addi x1, x2, 123"), @r#"
        [
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Ident(
                        "x2",
                    ),
                    Num(
                        123,
                    ),
                ],
                line: (
                    1,
                    "addi x1, x2, 123",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn expect_more_operand() {
        assert_debug_snapshot!(parse_source("addi x1,"), @r#"
        Err(
            "Error parsing line 1: 'addi x1,': Parsing requires more data",
        )
        "#
        );
        assert_debug_snapshot!(parse_source("addi x1, 123,"), @r#"
        Err(
            "Error parsing line 1: 'addi x1, 123,': Parsing requires more data",
        )
        "#
        );
    }

    #[test]
    fn expression_operand() {
        assert_debug_snapshot!(parse_ok("addi ra, x2, label + 4"), @r#"
        [
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "ra",
                    ),
                    Ident(
                        "x2",
                    ),
                    Addition(
                        "label",
                        4,
                    ),
                ],
                line: (
                    1,
                    "addi ra, x2, label + 4",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn offset_register_operand() {
        assert_debug_snapshot!(parse_ok("lw x1, (sp)"), @r#"
        [
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        0,
                    ),
                ],
                line: (
                    1,
                    "lw x1, (sp)",
                ),
            },
        ]
        "#
        );
        assert_debug_snapshot!(parse_ok("lw x1, 8(sp)"), @r#"
        [
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        8,
                    ),
                ],
                line: (
                    1,
                    "lw x1, 8(sp)",
                ),
            },
        ]
        "#
        );
        assert_debug_snapshot!(parse_ok("lw x1, (label + 8)(sp)"), @r#"
        [
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Ident(
                        "sp",
                    ),
                    Addition(
                        "label",
                        8,
                    ),
                ],
                line: (
                    1,
                    "lw x1, (label + 8)(sp)",
                ),
            },
        ]
        "#
        );
        assert_debug_snapshot!(parse_ok("lw x1, label + 8(sp)"), @r#"
        [
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Ident(
                        "sp",
                    ),
                    Addition(
                        "label",
                        8,
                    ),
                ],
                line: (
                    1,
                    "lw x1, label + 8(sp)",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn string_operand() {
        assert_debug_snapshot!(parse_ok(r#".string "hello world""#), @r#"
        [
            Instr {
                name: ".string",
                operands: [
                    String(
                        "hello world",
                    ),
                ],
                line: (
                    1,
                    ".string \"hello world\"",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn label_only() {
        assert_debug_snapshot!(parse_ok("loop: ; comment"), @r#"
        [
            Label(
                "loop",
            ),
        ]
        "#
        );
    }

    #[test]
    fn label_and_instruction_same_line() {
        assert_debug_snapshot!(parse_ok("loop: addi x1, x1, 1"), @r#"
        [
            Label(
                "loop",
            ),
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Ident(
                        "x1",
                    ),
                    Num(
                        1,
                    ),
                ],
                line: (
                    1,
                    "addi x1, x1, 1",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn comments_after_instruction() {
        assert_debug_snapshot!(
            parse_ok(
                r#"
                addi x1, x2, 1 # comment
                ; comment
                lw x3, 0(sp); comment
                "#
            ),
            @r#"
        [
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "x1",
                    ),
                    Ident(
                        "x2",
                    ),
                    Num(
                        1,
                    ),
                ],
                line: (
                    2,
                    "addi x1, x2, 1",
                ),
            },
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "x3",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        0,
                    ),
                ],
                line: (
                    4,
                    "lw x3, 0(sp)",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn multiple_labels() {
        assert_debug_snapshot!(parse_ok("foo: bar: nop"), @r#"
        [
            Label(
                "foo",
            ),
            Label(
                "bar",
            ),
            Instr {
                name: "nop",
                operands: [],
                line: (
                    1,
                    "nop",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn test_complex() {
        assert_debug_snapshot!(
            parse_ok(
                r#"
  .data
hello:
  .string "Hello, world!\n"

  .text
  .global main
main:
  li     a7, 64         # syscall: write
  li     a0, 1          # fd: stdout
  la     a1, hello      # buf
  li     a2, 14         # len
  ecall

  li     a7, 93         # syscall exit
  li     a0, 0          # exit code
  ecall
                "#
            ),
            @r#"
        [
            Instr {
                name: ".data",
                operands: [],
                line: (
                    2,
                    ".data",
                ),
            },
            Label(
                "hello",
            ),
            Instr {
                name: ".string",
                operands: [
                    String(
                        "Hello, world!\\n",
                    ),
                ],
                line: (
                    4,
                    ".string \"Hello, world!\\n\"",
                ),
            },
            Instr {
                name: ".text",
                operands: [],
                line: (
                    6,
                    ".text",
                ),
            },
            Instr {
                name: ".global",
                operands: [
                    Ident(
                        "main",
                    ),
                ],
                line: (
                    7,
                    ".global main",
                ),
            },
            Label(
                "main",
            ),
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "a7",
                    ),
                    Num(
                        64,
                    ),
                ],
                line: (
                    9,
                    "li     a7, 64",
                ),
            },
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Num(
                        1,
                    ),
                ],
                line: (
                    10,
                    "li     a0, 1",
                ),
            },
            Instr {
                name: "la",
                operands: [
                    Ident(
                        "a1",
                    ),
                    Ident(
                        "hello",
                    ),
                ],
                line: (
                    11,
                    "la     a1, hello",
                ),
            },
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "a2",
                    ),
                    Num(
                        14,
                    ),
                ],
                line: (
                    12,
                    "li     a2, 14",
                ),
            },
            Instr {
                name: "ecall",
                operands: [],
                line: (
                    13,
                    "ecall",
                ),
            },
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "a7",
                    ),
                    Num(
                        93,
                    ),
                ],
                line: (
                    15,
                    "li     a7, 93",
                ),
            },
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Num(
                        0,
                    ),
                ],
                line: (
                    16,
                    "li     a0, 0",
                ),
            },
            Instr {
                name: "ecall",
                operands: [],
                line: (
                    17,
                    "ecall",
                ),
            },
        ]
        "#
        );

        assert_debug_snapshot!(
            parse_ok(
                r#"
  .text
  .align  2

  .globl fib
fib:
  sw    ra, -4(sp)
  addi  sp, sp, -16
  li    t1, 2
  bgt   a0, t1, .l0
  li    a0, 1
  addi  sp, sp, 16
  lw    ra, -4(sp)
  ret
.l0:
  addi  s4, a0, -1
  sw    a0, 0(sp)
  mv    a0, s4
  call  fib
  mv    a3, a0
  lw    a0, 0(sp)
  addi  s4, a0, -2
  sw    a3, 0(sp)
  mv    a0, s4
  call  fib
  mv    s4, a0
  lw    a3, 0(sp)
  add   s4, a3, s4
  addi  sp, sp, 16
  lw    ra, -4(sp)

  mv    a0, s4
  ret

  .globl main
main:
  sw    ra, -4(sp)
  addi  sp, sp, -16

  call  getint
  call  fib
  call  putint

  li    a0, 10
  call  putch

  addi  sp, sp, 16
  lw    ra, -4(sp)

  li    a0, 0
  ret
                "#
            ),
            @r#"
        [
            Instr {
                name: ".text",
                operands: [],
                line: (
                    2,
                    ".text",
                ),
            },
            Instr {
                name: ".align",
                operands: [
                    Num(
                        2,
                    ),
                ],
                line: (
                    3,
                    ".align  2",
                ),
            },
            Instr {
                name: ".globl",
                operands: [
                    Ident(
                        "fib",
                    ),
                ],
                line: (
                    5,
                    ".globl fib",
                ),
            },
            Label(
                "fib",
            ),
            Instr {
                name: "sw",
                operands: [
                    Ident(
                        "ra",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        -4,
                    ),
                ],
                line: (
                    7,
                    "sw    ra, -4(sp)",
                ),
            },
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "sp",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        -16,
                    ),
                ],
                line: (
                    8,
                    "addi  sp, sp, -16",
                ),
            },
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "t1",
                    ),
                    Num(
                        2,
                    ),
                ],
                line: (
                    9,
                    "li    t1, 2",
                ),
            },
            Instr {
                name: "bgt",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Ident(
                        "t1",
                    ),
                    Ident(
                        ".l0",
                    ),
                ],
                line: (
                    10,
                    "bgt   a0, t1, .l0",
                ),
            },
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Num(
                        1,
                    ),
                ],
                line: (
                    11,
                    "li    a0, 1",
                ),
            },
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "sp",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        16,
                    ),
                ],
                line: (
                    12,
                    "addi  sp, sp, 16",
                ),
            },
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "ra",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        -4,
                    ),
                ],
                line: (
                    13,
                    "lw    ra, -4(sp)",
                ),
            },
            Instr {
                name: "ret",
                operands: [],
                line: (
                    14,
                    "ret",
                ),
            },
            Label(
                ".l0",
            ),
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "s4",
                    ),
                    Ident(
                        "a0",
                    ),
                    Num(
                        -1,
                    ),
                ],
                line: (
                    16,
                    "addi  s4, a0, -1",
                ),
            },
            Instr {
                name: "sw",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        0,
                    ),
                ],
                line: (
                    17,
                    "sw    a0, 0(sp)",
                ),
            },
            Instr {
                name: "mv",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Ident(
                        "s4",
                    ),
                ],
                line: (
                    18,
                    "mv    a0, s4",
                ),
            },
            Instr {
                name: "call",
                operands: [
                    Ident(
                        "fib",
                    ),
                ],
                line: (
                    19,
                    "call  fib",
                ),
            },
            Instr {
                name: "mv",
                operands: [
                    Ident(
                        "a3",
                    ),
                    Ident(
                        "a0",
                    ),
                ],
                line: (
                    20,
                    "mv    a3, a0",
                ),
            },
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        0,
                    ),
                ],
                line: (
                    21,
                    "lw    a0, 0(sp)",
                ),
            },
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "s4",
                    ),
                    Ident(
                        "a0",
                    ),
                    Num(
                        -2,
                    ),
                ],
                line: (
                    22,
                    "addi  s4, a0, -2",
                ),
            },
            Instr {
                name: "sw",
                operands: [
                    Ident(
                        "a3",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        0,
                    ),
                ],
                line: (
                    23,
                    "sw    a3, 0(sp)",
                ),
            },
            Instr {
                name: "mv",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Ident(
                        "s4",
                    ),
                ],
                line: (
                    24,
                    "mv    a0, s4",
                ),
            },
            Instr {
                name: "call",
                operands: [
                    Ident(
                        "fib",
                    ),
                ],
                line: (
                    25,
                    "call  fib",
                ),
            },
            Instr {
                name: "mv",
                operands: [
                    Ident(
                        "s4",
                    ),
                    Ident(
                        "a0",
                    ),
                ],
                line: (
                    26,
                    "mv    s4, a0",
                ),
            },
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "a3",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        0,
                    ),
                ],
                line: (
                    27,
                    "lw    a3, 0(sp)",
                ),
            },
            Instr {
                name: "add",
                operands: [
                    Ident(
                        "s4",
                    ),
                    Ident(
                        "a3",
                    ),
                    Ident(
                        "s4",
                    ),
                ],
                line: (
                    28,
                    "add   s4, a3, s4",
                ),
            },
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "sp",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        16,
                    ),
                ],
                line: (
                    29,
                    "addi  sp, sp, 16",
                ),
            },
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "ra",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        -4,
                    ),
                ],
                line: (
                    30,
                    "lw    ra, -4(sp)",
                ),
            },
            Instr {
                name: "mv",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Ident(
                        "s4",
                    ),
                ],
                line: (
                    32,
                    "mv    a0, s4",
                ),
            },
            Instr {
                name: "ret",
                operands: [],
                line: (
                    33,
                    "ret",
                ),
            },
            Instr {
                name: ".globl",
                operands: [
                    Ident(
                        "main",
                    ),
                ],
                line: (
                    35,
                    ".globl main",
                ),
            },
            Label(
                "main",
            ),
            Instr {
                name: "sw",
                operands: [
                    Ident(
                        "ra",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        -4,
                    ),
                ],
                line: (
                    37,
                    "sw    ra, -4(sp)",
                ),
            },
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "sp",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        -16,
                    ),
                ],
                line: (
                    38,
                    "addi  sp, sp, -16",
                ),
            },
            Instr {
                name: "call",
                operands: [
                    Ident(
                        "getint",
                    ),
                ],
                line: (
                    40,
                    "call  getint",
                ),
            },
            Instr {
                name: "call",
                operands: [
                    Ident(
                        "fib",
                    ),
                ],
                line: (
                    41,
                    "call  fib",
                ),
            },
            Instr {
                name: "call",
                operands: [
                    Ident(
                        "putint",
                    ),
                ],
                line: (
                    42,
                    "call  putint",
                ),
            },
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Num(
                        10,
                    ),
                ],
                line: (
                    44,
                    "li    a0, 10",
                ),
            },
            Instr {
                name: "call",
                operands: [
                    Ident(
                        "putch",
                    ),
                ],
                line: (
                    45,
                    "call  putch",
                ),
            },
            Instr {
                name: "addi",
                operands: [
                    Ident(
                        "sp",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        16,
                    ),
                ],
                line: (
                    47,
                    "addi  sp, sp, 16",
                ),
            },
            Instr {
                name: "lw",
                operands: [
                    Ident(
                        "ra",
                    ),
                    Ident(
                        "sp",
                    ),
                    Num(
                        -4,
                    ),
                ],
                line: (
                    48,
                    "lw    ra, -4(sp)",
                ),
            },
            Instr {
                name: "li",
                operands: [
                    Ident(
                        "a0",
                    ),
                    Num(
                        0,
                    ),
                ],
                line: (
                    50,
                    "li    a0, 0",
                ),
            },
            Instr {
                name: "ret",
                operands: [],
                line: (
                    51,
                    "ret",
                ),
            },
        ]
        "#
        )
    }
}
