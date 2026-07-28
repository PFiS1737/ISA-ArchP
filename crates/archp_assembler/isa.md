# ISA

## Types

### R-type

```
xxxxxxx    000   xxxxx   xxxxx   0000000   xxxxx
 opcode |funct3|   rd  |  rs1  |   ---   |  rs2
```

### I-type

```
xxxxxxx    000   xxxxx   xxxxx   xxxxxxxxxxxx
 opcode |funct3|   rd  |  rs1  |    imm12
```

### B/S-type

```
xxxxxxx    000   xxxxx   xxxxx   xxxxxxx   xxxxx
 opcode |funct3|  hi5  |  rs1  |   lo7   |  rs2  (offset12 = hi5 << 7 | lo7)
```

### U/J-type

```
xxxxxxx   xxx   xxxxx   xxxxxxxxxxxxxxxxx
 opcode | hi3 |  rd  |        low17       (uimm20 = hi3 << 17 | lo17)
```

## Instructions

> [!NOTE]
> For more detailed encoding formats, please refer to the source code files,
> such as [stack_call_return.rs](./src/instructions/stack_call_return.rs).

### Arithmetic / Logic

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0000000 | 000    | R    | add         |
| 0000000 | 001    | R    | sub         |
| 0000000 | 010    | R    | and         |
| 0000000 | 011    | R    | or          |
| 0000000 | 100    | I    | addi        |
| 0000000 | 101    | I    | subi        |
| 0000000 | 110    | I    | andi        |
| 0000000 | 111    | I    | ori         |
| 0000001 | 000    | R    | xor         |
| 0000001 | 001    | R    | xnor        |
| 0000001 | 010    | R    | nand        |
| 0000001 | 011    | R    | nor         |
| 0000001 | 100    | I    | xori        |
| 0000001 | 101    | I    | xnori       |
| 0000001 | 110    | I    | nandi       |
| 0000001 | 111    | I    | nori        |

### Multiply

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0000010 | 000    | R    | mul         |
| 0000010 | 001    | R    | mulh        |
| 0000010 | 010    | R    | mulhu       |
| 0000010 | 011    | R    | mulhsu      |
| 0000010 | 100    | I    | muli        |
| 0000010 | 101    | I    | mulhi       |
| 0000010 | 110    | I    | mulhiu      |
| 0000010 | 111    | I    | mulhisu     |

### Division / Remainder

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0000011 | 000    | R    | div         |
| 0000011 | 001    | R    | rem         |
| 0000011 | 010    | R    | divu        |
| 0000011 | 011    | R    | remu        |
| 0000011 | 100    | I    | divi        |
| 0000011 | 101    | I    | remi        |
| 0000011 | 110    | I    | diviu       |
| 0000011 | 111    | I    | remiu       |

### Shift / Rotate

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0000100 | 000    | R    | sll         |
| 0000100 | 001    | R    | srl         |
| 0000100 | 010    | -    | -           |
| 0000100 | 011    | R    | sra         |
| 0000100 | 100    | R    | rol         |
| 0000100 | 101    | R    | ror         |
| 0000100 | 110    | -    | -           |
| 0000100 | 111    | -    | -           |
| 0000101 | 000    | I    | slli        |
| 0000101 | 001    | I    | srli        |
| 0000101 | 010    | -    | -           |
| 0000101 | 011    | I    | srai        |
| 0000101 | 100    | I    | roli        |
| 0000101 | 101    | I    | rori        |
| 0000101 | 110    | -    | -           |
| 0000101 | 111    | -    | -           |

### Set

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0000110 | 000    | R    | seq         |
| 0000110 | 001    | R    | sne         |
| 0000110 | 010    | R    | slt         |
| 0000110 | 011    | R    | sge         |
| 0000110 | 100    | R    | sltu        |
| 0000110 | 101    | R    | sgeu        |
| 0000110 | 110    | -    | -           |
| 0000110 | 111    | -    | -           |
| 0000111 | 000    | I    | seqi        |
| 0000111 | 001    | I    | snei        |
| 0000111 | 010    | I    | slti        |
| 0000111 | 011    | I    | sgei        |
| 0000111 | 100    | I    | sltiu       |
| 0000111 | 101    | I    | sgeiu       |
| 0000111 | 110    | -    | -           |
| 0000111 | 111    | -    | -           |

### Load / Store

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0001000 | 000    | I    | lw          |
| 0001000 | 001    | I    | lh          |
| 0001000 | 010    | I    | lhu         |
| 0001000 | 011    | I    | lb          |
| 0001000 | 100    | I    | lbu         |
| 0001000 | 101    | S    | sw          |
| 0001000 | 110    | S    | sh          |
| 0001000 | 111    | S    | sb          |

### Branch

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0001001 | 000    | B    | beq         |
| 0001001 | 001    | B    | bne         |
| 0001001 | 010    | B    | blt         |
| 0001001 | 011    | B    | bge         |
| 0001001 | 100    | B    | bltu        |
| 0001001 | 101    | B    | bgeu        |
| 0001001 | 110    | -    | -           |

### Jump and Link

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0001001 | 111    | I    | jalr        |
| 0001010 | -      | J    | jal         |

### Upper Immediate

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 0001011 | -      | U    | lui         |
| 0001100 | -      | U    | auipc       |

### Stack / Call / Return

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 1111101 | -      | J    | call        |
| 1111110 | 000    | I    | pop         |
| 1111110 | 001    | I    | push        |
| 1111110 | 010    | I    | ret         |
| 1111110 | 011    | I    | callr       |

### Misc

| Opcode  | Funct3 | Type | Instruction |
| ------- | ------ | ---- | ----------- |
| 1111111 | 000    | I    | colr        |
| 1111111 | 001    | R    | spx         |
| 1111111 | 010    | I    | in          |
| 1111111 | 011    | I    | out         |
| 1111111 | 101    | I    | rand        |
