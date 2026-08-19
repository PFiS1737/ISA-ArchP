j main

hello:
  .asciz "Hello"

main:
  la a0, hello
  ecall 4 ; print string

  la a0, world
  ecall 4 ; print string

  ecall 10 ; exit

world:
  .asciz " World!\n"
