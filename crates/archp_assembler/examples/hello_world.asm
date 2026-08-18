la a0, hello
ecall 4 ; print string
ecall 10 ; exit

hello:
  .asciz "Hello World!\n"
