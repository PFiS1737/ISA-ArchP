j main

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
  ecall 4 ; print string

  la a0, world
  ecall 4 ; print string

  ecall 10 ; exit

zero2:
  .zero 2048

world:
  .asciz " World!\n"
