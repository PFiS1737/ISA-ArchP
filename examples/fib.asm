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
    mv io a0

fib:
    bgt a0 2 .L8
    li a0 1
    jmpr ra
.L8:
    sub sp sp 3
    sw ra 2(sp)
    sw s0 1(sp)
    sw s1 0(sp)
    mv s0 a0
    add a0 a0 -1
    jal ra fib
    mv s1 a0
    add a0 s0 -2
    jal ra fib
    add a0 s1 a0
    lw ra 2(sp)
    lw s0 1(sp)
    lw s1 0(sp)
    add sp sp 3
    jmpr ra
