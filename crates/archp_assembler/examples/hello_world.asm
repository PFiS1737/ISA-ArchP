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

  la t3, test_load_store
  jalr t3

  li a7, 10 ; exit
  ecall

zero2:
  .zero 2048

world:
  .asciz " World!\n"

test_load_store:
  lw a0, zero1
  li a7, 1 ; print int
  ecall ; 0x02010101 == 33620225

  lb a0, hello
  li a7, 11 ; print char
  ecall ; 'H'

  lb a0, world+7
  li a7, 11 ; print char
  ecall ; '\n'

  li a0, 65
  sb a0, hello, t0
  la a0, hello
  li a7, 4 ; print string
  ecall ; 'Aello'
  ret
