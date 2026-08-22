use anyhow::{anyhow, bail};
use nom::{
    Parser,
    branch::alt,
    character::complete::{char, space0, space1},
    combinator::{map, opt},
    multi::separated_list0,
    sequence::{preceded, terminated},
};
use smallvec::SmallVec;

use crate::{
    assembler::Line,
    context::Context,
    directives::DirectiveOperand,
    operand::Operand,
    parser::{Result, expression::expr, identifier::ident, operand::operand, string::string, ws},
};

fn label(input: &str) -> Result<'_, &str> {
    terminated(alt((string, ident)), (char(':'), space0)).parse(input)
}

fn operands<'ctx, 'src: 'ctx>(
    ctx: &'ctx Context<'src>,
    input: &'src str,
) -> Result<'src, SmallVec<[Operand<'src>; 3]>> {
    let mut out = SmallVec::new();

    let (input, o) = opt(preceded(space1, |input| operand(ctx, input, &mut out))).parse(input)?;

    if o.is_none() {
        return Ok((input, out));
    }

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

fn directive_operands<'src>(input: &'src str) -> Result<'src, Vec<DirectiveOperand<'src>>> {
    opt(preceded(
        space1,
        separated_list0(
            ws(char(',')),
            alt((
                map(string, DirectiveOperand::String),
                map(expr, DirectiveOperand::Expr),
                map(space0, |_| DirectiveOperand::Empty),
            )),
        ),
    ))
    .parse(input)
    .map(|(i, opt)| match opt {
        Some(v) => (i, v),
        None => (i, Vec::new()),
    })
}

fn line<'ctx, 'src: 'ctx>(
    ctx: &'ctx Context<'src>,
    line_num: usize,
    input: &'src str,
) -> Result<'src, Line<'src>> {
    let raw = input;

    let (input, name) = ident(input)?;

    if name.starts_with('.') {
        let (input, ops) = directive_operands(input)?;

        Ok((input, Line::Directive {
            name,
            operands: ops,
            line: (line_num, raw),
        }))
    } else {
        let (input, ops) = operands(ctx, input)?;

        Ok((input, Line::Instruction {
            name,
            operands: ops,
            line: (line_num, raw),
        }))
    }
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
    input: &'src str,
) -> anyhow::Result<SmallVec<[Line<'src>; 2]>> {
    let input = strip_comment(input).trim();

    if input.is_empty() {
        return Ok(SmallVec::new());
    }

    let mut out = SmallVec::new();
    let mut rest = input;

    while let Ok((r, l)) = label(rest) {
        out.push(Line::Label(l));
        rest = r;
    }

    if rest.is_empty() {
        return Ok(out);
    }

    let (remain, line) = line(ctx, line_num, rest)
        .map_err(|e| anyhow!("Error parsing line {}: '{}': {}", line_num, input, e))?;

    if !remain.is_empty() {
        bail!("Unexpected content after line {}: '{}'", line_num, remain);
    }

    out.push(line);

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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            "Error parsing line 1: 'addi x1,': Parsing Error: Nom(Error { input: \"\", code: Char })",
        )
        "#
        );
        assert_debug_snapshot!(parse_source("addi x1, 123,"), @r#"
        Err(
            "Error parsing line 1: 'addi x1, 123,': Parsing Error: Nom(Error { input: \"\", code: Char })",
        )
        "#
        );
    }

    #[test]
    fn expression_operand() {
        assert_debug_snapshot!(parse_ok("addi ra, x2, label + 4"), @r#"
        [
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Directive {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
    fn string_label() {
        assert_debug_snapshot!(
            parse_ok(
                r#"
"fib(int n)":
  j "fib(int n)"
                "#
        ),
        @r#"
        [
            Label(
                "fib(int n)",
            ),
            Instruction {
                name: "j",
                operands: [
                    Ident(
                        "fib(int n)",
                    ),
                ],
                line: (
                    3,
                    "j \"fib(int n)\"",
                ),
            },
        ]
        "#
        )
    }

    #[test]
    fn omit_directive_operands() {
        assert_debug_snapshot!(parse_ok(".align 2 , , max"), @r#"
        [
            Directive {
                name: ".align",
                operands: [
                    Expr(
                        Num(
                            2,
                        ),
                    ),
                    Empty,
                    Expr(
                        Ident(
                            "max",
                        ),
                    ),
                ],
                line: (
                    1,
                    ".align 2 , , max",
                ),
            },
        ]
        "#
        );
    }

    #[test]
    fn test_complex_1() {
        assert_debug_snapshot!(
            parse_source(
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
        Ok(
            [
                Directive {
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
                Directive {
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
                Directive {
                    name: ".text",
                    operands: [],
                    line: (
                        6,
                        ".text",
                    ),
                },
                Directive {
                    name: ".global",
                    operands: [
                        Expr(
                            Ident(
                                "main",
                            ),
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
                Instruction {
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
                Instruction {
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
                Instruction {
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
                Instruction {
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
                Instruction {
                    name: "ecall",
                    operands: [],
                    line: (
                        13,
                        "ecall",
                    ),
                },
                Instruction {
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
                Instruction {
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
                Instruction {
                    name: "ecall",
                    operands: [],
                    line: (
                        17,
                        "ecall",
                    ),
                },
            ],
        )
        "#
        );
    }

    #[test]
    fn test_complex_2() {
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
            Directive {
                name: ".text",
                operands: [],
                line: (
                    2,
                    ".text",
                ),
            },
            Directive {
                name: ".align",
                operands: [
                    Expr(
                        Num(
                            2,
                        ),
                    ),
                ],
                line: (
                    3,
                    ".align  2",
                ),
            },
            Directive {
                name: ".globl",
                operands: [
                    Expr(
                        Ident(
                            "fib",
                        ),
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
                name: "ret",
                operands: [],
                line: (
                    33,
                    "ret",
                ),
            },
            Directive {
                name: ".globl",
                operands: [
                    Expr(
                        Ident(
                            "main",
                        ),
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
            Instruction {
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
