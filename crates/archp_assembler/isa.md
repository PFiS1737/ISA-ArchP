# ISA

## Types

### R-type

```
xxxx xxx   000   xxxxx   xxxxx   0000000   xxxxx
 opcode  |  -  |   rd  |  rs1  |   ---   |  rs2
```

### I-type

```
xxxx xxx   000   xxxxx   xxxxx   xxxxxxxxxxxx
 opcode  |  -  |   rd  |  rs1  |    imm12
```

### B/S-type

```
xxxx xxx   000   xxxxx   xxxxx   xxxxxxx   xxxxx
 opcode  |  -  |  hi5  |  rs1  |   lo7   |  rs2  (offset12 = hi5 << 7 | lo7)
```

### U/J-type

```
xxxx xxx   xxx   xxxxx   xxxxxxxxxxxxxxxxx
 opcode  | hi3 |  rd  |        low17       (uimm20 = hi3 << 17 | lo17)
```

### C-type

```
1101 000   0   xxxxxxxx xxxxxxxx xxxxxxxx
   col   | - |          color24
```

## Instructions

> [!NOTE]
> For more detailed encoding formats, please refer to the source code files,
> such as [stack_call_return.rs](./src/instructions/stack_call_return.rs).

### register compute

| Encoding | Instruction | Type | Notes |
|----------|-------------|------|-------|
| 0000 000 | add | R | add |
| 0000 001 | sub | R | sub |
| 0000 010 | mul | R | mul |
| 0000 011 | mulh | R | mul high |
| 0000 100 | mulhu | R | mul high unsigned |
| 0000 101 | mulhsu | R | mul high mixed |
| 0000 110 | rem | R | remainder |
| 0000 111 | div | R | division |
| 0001 000 | and | R | and |
| 0001 001 | nand | R | nand |
| 0001 010 | or | R | or |
| 0001 011 | nor | R | nor |
| 0001 100 | xor | R | xor |
| 0001 101 | xnor | R | xnor |
| 0010 000 | sll | R | shift left |
| 0010 001 | srl | R | shift right |
| 0010 010 | rol | R | rotate left |
| 0010 011 | ror | R | rotate right |
| 0010 100 | sra | R | arithmetic shift right |
| 0011 001 | seq | R | equal |
| 0011 010 | sne | R | not equal |
| 0011 011 | slt | R | less than |
| 0011 100 | sge | R | greater or equal |
| 0011 101 | sltu | R | less than unsigned |
| 0011 110 | sgeu | R | greater or equal unsigned |

### immediate compute

| Encoding | Instruction | Type | Notes |
|----------|------------|------|-------|
| 0100 000 | addi | I | add |
| 0100 001 | subi | I | sub |
| 0100 010 | muli | I | mul |
| 0100 011 | mulhi | I | mul high |
| 0100 100 | mulhiu | I | mul high unsigned |
| 0100 101 | mulhisu | I | mul high mixed |
| 0100 110 | remi | I | remainder |
| 0100 111 | divi | I | division |
| 0101 000 | andi | I | and |
| 0101 001 | nandi | I | nand |
| 0101 010 | ori | I | or |
| 0101 011 | nori | I | nor |
| 0101 100 | xori | I | xor |
| 0101 101 | xnori | I | xnor |
| 0110 000 | slli | I | shift left |
| 0110 001 | srli | I | shift right |
| 0110 010 | roli | I | rotate left |
| 0110 011 | rori | I | rotate right |
| 0110 100 | srai | I | arithmetic shift right |
| 0111 001 | seqi | I | equal |
| 0111 010 | snei | I | not equal |
| 0111 011 | slti | I | less than |
| 0111 100 | sgei | I | greater or equal |
| 0111 101 | sltiu | I | less than unsigned |
| 0111 110 | sgeiu | I | greater or equal unsigned |

### load and store

| Encoding | Instruction | Type | Notes |
|----------|------------|------|-------|
| 1000 000 | lw | I | load word |
| 1000 001 | lh | I | load half |
| 1000 010 | lhu | I | load half unsigned |
| 1000 011 | lb | I | load byte |
| 1000 100 | lbu | I | load byte unsigned |
| 1000 101 | sw | S | store word |
| 1000 110 | sh | S | store half |
| 1000 111 | sb | S | store byte |

### jump and branch

| Encoding | Instruction | Type | Notes |
|----------|------------|------|-------|
| 1001 000 | jal | J | jump and link |
| 1001 001 | beq | B | equal |
| 1001 010 | bne | B | not equal |
| 1001 011 | blt | B | less than |
| 1001 100 | bge | B | greater or equal |
| 1001 101 | bltu | B | less than unsigned |
| 1001 110 | bgeu | B | greater or equal unsigned |
| 1001 111 | jalr | I | jump and link register |

### stack, return and call

| Encoding | Instruction | Type | Notes |
|----------|------------|------|-------|
| 1010 000 | pop | I | pop |
| 1010 001 | push | I | push |
| 1010 100 | ret | I | return |
| 1010 101 | call | J | call |
| 1010 110 | callr | I | call register |

### upper immediate

| Encoding | Instruction | Type | Notes |
|----------|------------|------|-------|
| 1011 000 | lui | U | load upper immediate |
| 1011 001 | auipc | U | add upper immediate to pc |

### misc

| Encoding | Instruction | Type | Notes |
|----------|------------|------|-------|
| 1101 000 | col | C | set color |
| 1101 001 | spx | R | set pixel |
| 1101 010 | seg | R | segment display |
| 1101 011 | segi | I | segment display immediate |
