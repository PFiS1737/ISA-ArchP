# The insertion sort algorithm implemented in assembly.

const zero = r0
const ra = r1
const sp = r2
const t0 = r5
const t1 = r6
const t2 = r7
const s0 = r8
const a0 = r10
const a1 = r11
const a2 = r12
const a3 = r13
const a4 = r14
const a5 = r15
const t3 = r28
const t4 = r29
const t5 = r30

li sp, 8192

j main

# a0 = int*
# a1 = length
insertion_sort:
   li t0, 1  # i = 1

  .Linsertion_sort_outer:
    bge t0, a1, .Linsertion_sort_done  # if i >= length: done

    # t2 = a[i]
    sll t1, t0, 2  # offset = i * 4
    add t1, a0, t1
    lw t2, 0(t1)

    add t3, t0, -1  # j = i - 1

    .Linsertion_sort_inner:
      blt t3, zero, .Linsertion_sort_insert

      # t5 = a[j]
      sll t4, t3, 2
      add t4, a0, t4
      lw t5, 0(t4)

      ble t5, t2, .Linsertion_sort_insert

      # a[j+1] = a[j]
      sw t5, 4(t4)

      dec t3
      j .Linsertion_sort_inner

    .Linsertion_sort_insert:
      # a[j+1] = t2
      inc t3
      sll t3, t3, 2
      add t3, a0, t3
      sw t2, 0(t3)

      inc t0
      j .Linsertion_sort_outer

  .Linsertion_sort_done:
    ret

main:
  ecall 5 ; read int
  mv a1, a0 # length

  # malloc from stack frame
  sll s0, a1, 2  # size = length * 4
  sub sp, sp, s0

  mv t3, a1
  mv t1, sp

  .Lmain_input_loop:
    beqz t3, .Lmain_input_done

    ecall 5 ; read int
    mv t2, a0

    sw t2, 0(t1)
    add t1, t1, 4

    dec t3
    j .Lmain_input_loop

  .Lmain_input_done:
    mv a0, sp
    call insertion_sort
    mv t0, a0

  .Lmain_output_loop:
    beq a1, zero, .Lmain_output_done

    lw a0, 0(t0)
    ecall 1 ; print int
    li a0, 10 ; '\n'
    ecall 11 ; putchar
    add t0, t0, 4

    dec a1
    j .Lmain_output_loop

  .Lmain_output_done:
    add sp, sp, s0
    j halt

halt:
  ecall 10 ; exit
