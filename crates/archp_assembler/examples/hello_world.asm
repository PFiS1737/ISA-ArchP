li r1, 4096

li r2, 72
sb r2, 0(r1)

li r2, 101
sb r2, 1(r1)

li r2, 108
sb r2, 2(r1)

li r2, 108
sb r2, 3(r1)

li r2, 111
sb r2, 4(r1)

li r2, 32
sb r2, 5(r1)

li r2, 87
sb r2, 6(r1)

li r2, 111
sb r2, 7(r1)

li r2, 114
sb r2, 8(r1)

li r2, 108
sb r2, 9(r1)

li r2, 100
sb r2, 10(r1)

li r2, 33
sb r2, 11(r1)

li r2, 10
sb r2, 12(r1)

li r2, 0
sb r2, 13(r1)

mv r10, r1
ecall 4 ; print string

ecall 10 ; exit
