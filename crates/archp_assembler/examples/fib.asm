const s0 r1
const s1 r2
const s2 r3

const a0 r4
const ra r5

const sp r6

li sp 4096

main:
  li a0 10
  jal ra fib
  out a0

fib:
  bgt a0 2 .L0
  li a0 1
  jr ra
.L0:
  sub sp sp 12
  sw ra 8(sp)
  sw s0 4(sp)
  sw s1 0(sp)
  mv s0 a0
  add a0 a0 -1
  jal ra fib
  mv s1 a0
  add a0 s0 -2
  jal ra fib
  add a0 s1 a0
  lw ra 8(sp)
  lw s0 4(sp)
  lw s1 0(sp)
  add sp sp 12
  jr ra
