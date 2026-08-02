# System Calls

### 1. Print Integer

#### Arguments

- `a0`: integer to print

### 4. Print String

#### Arguments

- `a0`: pointer to the null-terminated string to print

### 5. Read Integer

> [!CAUTION]
> This syscall uses `scanf`, which reads from the buffered `stdin` stream.
> Do not mix it with `63: Read`, which directly invokes the host OS `read` syscall. The two interfaces do not share the same buffering state.

#### Returns

- `a0`: the integer read from input

### 10. Exit

### 11. Print Character

#### Arguments

- `a0`: character to print (lower 8 bits are used)

### 12. Read Character

> [!CAUTION]
> This syscall uses `getchar`, which reads from the buffered `stdin` stream.
> Do not mix it with `63: Read`, which directly invokes the host OS `read` syscall. The two interfaces do not share the same buffering state.

#### Returns

- `a0`: the return value of `getchar`

### 41. Random Integer

#### Arguments

- `a0`: ID of pseudorandom number generator (not used in the current implementation)

#### Returns

- `a0`: a pseudorandom integer

### 42. Random Integer Range

#### Arguments

- `a0`: ID of pseudorandom number generator (not used in the current implementation)
- `a1`: upper bound (exclusive)

#### Returns

- `a0`: a pseudorandom integer in the range `[0, a1)`

### 63. Read

#### Arguments

- `a0`: file descriptor
- `a1`: pointer to the buffer to read into
- `a2`: number of bytes to read

#### Returns

- `a0`: number of bytes read, or -1 on error

### 64. Write

#### Arguments

- `a0`: file descriptor
- `a1`: pointer to the buffer to write from
- `a2`: number of bytes to write

#### Returns

- `a0`: number of bytes written, or -1 on error

### 0x1000_0000. Set Pixel

#### Arguments

- `a0`: x coordinate
- `a1`: y coordinate
- `a2`: color (0xRRGGBB)
