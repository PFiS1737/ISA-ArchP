j main

hello:
  .asciz "Hello"

zero1:
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
