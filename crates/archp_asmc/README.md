## CLI Usage

```
Assembly implementation of the ArchP ISA

Usage: archp_asmc [OPTIONS] [SRC_FILE]

Arguments:
  [SRC_FILE]  File path to the source assembly file

Options:
      --complete <COMPLETE>  Print shell auto completions for the specified shell [possible values: bash, elvish, fish, powershell, zsh]
  -o, --output <OUTPUT>      The output file path [default: <stdout>]
      --bin                  Output binary machine code instead of formatted hex
      --disable-macro        Disable the macro-instructions
  -h, --help                 Print help
  -V, --version              Print version
```

## Assembly Syntax

- Comments start with `#` or `;` and continue to the end of the line.
- Definite constants using `const` directive: `const NAME VALUE` (only allowed at the beginning of the file).
- Labels are defined by writing the label name followed by a colon (`:`) at the beginning of a line.
- Operands are separated by spaces, tabs or commas.
- Instructions and labels are case-insensitive.
- Only one instruction or label definition is allowed per line (label and instruction can be on the same line).
- See [examples](./examples) for more details.

## ISA

This section is intended for users.
If you require detailed information on the encoding formats, please refer to [isa.txt](./isa.txt).

### Registers

- 24 general-purpose registers: `r1` to `r24`.
- special registers:
  - `r0`: always contains 0, you can write to it but it has no effect.
  - `io`: the level input/output (only in level mode).
  - `kb`: the keyboard input (only in sandbox mode, read-only).
  - `rng`: the random number generator (read-only).
  - `tmp`: used by the assembler for expanding macro-instructions.

### Instructions

In this section:

- `instr`: the instruction name.
- `rd`: the destination register.
- `rs1`: the first source register.
- `rs2`: the second source register.
- `imm12`: 12-bit signed immediate value, from `-2048` to `2047`.
- `addr12`, `addr20`: 12/20-bit signed relative address offset in the unit of 2-byte.
  - This usually indicates: `pc = pc + sign_extend(addr << 1)`.
- `immX`: will be specified in the instruction description.
- numeric literal: `42`, `-7`, `0b010101`, `0xFE42`, etc.

> [!NOTE]
> The 'signed' and 'unsigned' are merely formal distinctions,
> you can always use `0xFFFFFFFE` (or `0xFFE` in 12 bits) to represent `-2`.

> [!NOTE]
> You can not write `addr12` directly as a numeric literal, you must use a label.

> [!NOTE]
> The `macro` features are enabled by default.
> You can disable them using the `--disable-macro` option when invoking the assembler.

#### Arithmetic

- instructions:
  - `add`, `sub`, `mul`, `mulh`, `mulhu`, `mulhsu`, `div`, `rem`
  - `addi`, `subi`, `muli`, `mulhi`, `mulhiu`, `mulhisu`, `divi`, `remi`
- format: `instr rd rs1 rs2/imm12`
- pseudo:
  - `li rd imm12` => `addi rd r0 imm12`
  - `mv rd rs1` => `addi rd rs1 0`
  - `inc rd` => `addi rd rd 1`
  - `dec rd` => `subi rd rd 1`
  - `clr rd` => `addi rd r0 0`
  - `neg rd rs1` => `sub rd r0 rs1`
- macros:
  - When using register series instructions, if the third operand is a numeric literal, it will be automatically replaced with an immediate series instruction.
  - When using immediate series instructions, if the immediate is larger the 12-bit, it will be automatically expanded into multiple instructions to load the immediate into a temporary register first.
  - e.g. `sub r1 r2 0x1234` => `subi r1 r2 0x1234` => `lui tmp 0x1; addi tmp tmp 0x234; sub r1 r2 tmp`

#### Logical

- instructions:
  - `and`, `or`, `xor`, `nand`, `nor`, `xnor`
  - `andi`, `ori`, `xori`, `nandi`, `nori` `xnori`
- format: `instr rd rs1 rs2/imm12`
- pseudo:
  - `not rd rs1` => `xori rd rs1 -1`
- macros: Same as the [Arithmetic](#arithmetic) instructions.

#### Shift and Rotate

- instructions: `sll`, `srl`, `rol`, `ror`, `sra`, `slli`, `srli`, `roli`, `rori`, `srai`
- format: `instr rd rs1 rs2/imm5`, where `imm5` is a 5-bit unsigned immediate value from `0` to `31`.
- macros: Same as the [Arithmetic](#arithmetic) instructions.

#### Set

- instructions:
  - `seq`, `sne`, `slt`, `sge`, `sltu`, `sgeu`
  - `seqi`, `snei`, `slti`, `sgei`, `sltiu`, `sgeiu`
- format: `instr rd rs1 rs2/imm12`
- pseudo:
  - `sgt rd rs1 rs2` => `slt rd rs2 rs1`
  - `sle rd rs1 rs2` => `sge rd rs2 rs1`
  - `sgtu rd rs1 rs2` => `sltu rd rs2 rs1`
  - `sleu rd rs1 rs2` => `sgeu rd rs2 rs1`
  - `s**z rd rs1` => `s** rd rs1 r0` (for `seqz`, `snez`, `sltz`, `sgez`, `sgtz`, `slez`)
- macros:
  - Same as the [arithmetic](#arithmetic) instructions, but with special handling for the pseudo-instructions like `sgt`.
  - e.g. `slt r1 r2 0x1234` => `slti r1 r2 0x1234` => `lui tmp 0x1; addi tmp tmp 0x234; slt r1 r2 tmp`
  - e.g. `sgt r1 r2 0x1234` => `li tmp 0x1234; sgt r1 r2 tmp` => `lui tmp 0x1; addi tmp tmp 0x234; slt r1 tmp r2`

#### Load and Store

- `lw rd rs1 imm12`: load word from memory address `rs1 + imm12` into `rd`.
- `lh rd rs1 imm12`: load half-word from memory address `rs1 + imm12` into the lower 16 bits of `rd`, sign-extended.
- `lhu rd rs1 imm12`: load half-word from memory address `rs1 + imm12` into the lower 16 bits of `rd`, zero-extended.
- `lb rd rs1 imm12`: load byte from memory address `rs1 + imm12` into the lower 8 bits of `rd`, sign-extended.
- `lbu rd rs1 imm12`: load byte from memory address `rs1 + imm12` into the lower 8 bits of `rd`, zero-extended.
- `sw rs1 rs2 imm12`: store word from `rs2` into memory address `rs1 + imm12`.
- `sh rs1 rs2 imm12`: store the lower 16 bits of `rs2` into memory address `rs1 + imm12`.
- `sb rs1 rs2 imm12`: store the lower 8 bits of `rs2` into memory address `rs1 + imm12`.
- note:
  - The unit of memory address is in bytes.
- macros:
  - It allows you to use a RISC-V-style offset syntax.
  - e.g. `lw rd rs1 4` => `lw rd 4(rs1)`; `sw rs1 rs2 -8` => `sw rs2 -8(rs1)`

> [!CAUTION]
> Please ensure that the memory addresses accessed by load and store instructions are properly aligned:
> - `lw`, `sw`: 4-byte
> - `lh`, `lhu`, `sh`: 2-byte
> - `lb`, `lbu`, `sb`: 1-byte (no alignment needed)
> Unaligned memory accesses is an undefined behavior at the hardware level and may lead to unexpected results.

> [!NOTE]
> If you are simulating the hardware stack using a stack pointer register (e.g. `const sp r10`),  
> remember to initialize it to point to the top of the stack memory region. (e.g. `li sp 4096`)  
> Example: [fib.asm](./examples/fib.asm)

#### Upper Immediate

- `lui rd imm20`: `rd = imm20 << 12`.
- `auipc rd imm20`: `rd = pc + (imm20 << 12)`.

#### Branching

- instructions: `beq`, `bne`, `blt`, `bge`, `bltu`, `bgeu`
- format: `instr rs1 rs2 addr12`
- pseudo:
  - `bgt rs1 rs2 addr12` => `blt rs2 rs1 addr12`
  - `ble rs1 rs2 addr12` => `bge rs2 rs1 addr12`
  - `bgtu rs1 rs2 addr12` => `bltu rs2 rs1 addr12`
  - `bleu rs1 rs2 addr12` => `bgeu rs2 rs1 addr12`
  - `b**z rs1 addr12` => `b** rs1 r0 addr12` (for `beqz`, `bnez`, `bltz`, `bgez`, `bgtz`, `blez`)
- macros:
  - If the `rs2` operand is a numeric literal, it will be automatically expanded to use a temporary register.
  - A 32-bit immediate literal is also supported.
  - e.g. `beq r1 0x1234 10` => `li tmp 0x1234; beq r1 tmp 10` => `lui tmp 0x1; addi tmp tmp 0x234; beq r1 tmp 10`

#### Stack Operations

- `push rs1`: push the value of `rs1` onto the stack.
- `pop rd`: pop the top value from the stack into `rd`.

#### Call and Return

- `call addr20`: call a subroutine at an address.
- `callr rs1 imm12`: call a subroutine at the address `(rs1 + sign_extend(imm12)) & ~1`.
- `ret`: return from the current subroutine.
- note:
  - The return address is automatically managed by the hardware stack.

#### Jump and Link

- `jal rd addr20`: jump to an address and write the return address (`pc + 4`) into `rd`.
- `jalr rd rs1 imm12`: jump to the address `(rs1 + sign_extend(imm12)) & ~1` and write the return address (`pc + 4`) into `rd`.
- pseudo:
  - `j addr12` => `jal r0 addr12`
  - `jr rs1` => `jalr r0 rs1`
- note:
  - You may know how to use this instruction if you are familiar with the RISC-V architecture.

#### Display (only in sandbox mode)

- `col imm24u`: set the display color to the 24-bit unsigned immediate value `imm24u` (format: `0xRRGGBB`).
- `spx rs1 rs2`: set the `(rs1, rs2)` position to the color specified by the last `col` instruction.
- `seg rs1`: display the value of `rs1` on a 7-segment display.
- `segi imm12`: display the immediate value on a 7-segment display.
