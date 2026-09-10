jump main, t0

hello:
  .asciz "Hello"

zero1:
  .byte 1, 1, 1
  .half 2, 2, 2
  .word 3, 3, 3
  .align 2, 4
  .zero 2048

main:
  la a0, hello
  li a7, 4 ; print string
  ecall

  la a0, world
  li a7, 4 ; print string
  ecall

  li a7, 10 ; exit
  ecall

zero2:
  .zero 2048

world:
  .asciz " World!\n"
