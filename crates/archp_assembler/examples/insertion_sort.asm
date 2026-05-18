# The insertion sort algorithm implemented in assembly.

# Also demonstrates reading multiple digits from the keyboard,
# accumulating them into an integer, and displaying integers on the 7-segment display.

const zero r0

const sp r1

const a0 r2
const a1 r3
const a2 r4
const a3 r5
const a4 r6
const a5 r7

const t0 r8
const t1 r9
const t2 r10
const t3 r11
const t4 r12
const t5 r13

const s0 r14

const KEY_1 14
const KEY_2 15
const KEY_3 16
const KEY_4 17
const KEY_5 18
const KEY_6 19
const KEY_7 20
const KEY_8 21
const KEY_9 22
const KEY_0 23
const KEY_ENTER 53

li sp 8192

j main

# a0 = int*
# a1 = length
insertion_sort:
   li t0 1  # i = 1

  .Linsertion_sort_outer:
    bge t0 a1 .Linsertion_sort_done  # if i >= length: done

    # t2 = a[i]
    sll t1 t0 2  # offset = i * 4
    add t1 a0 t1
    lw t2 0(t1)

    add t3 t0 -1  # j = i - 1

    .Linsertion_sort_inner:
      blt t3 zero .Linsertion_sort_insert

      # t5 = a[j]
      sll t4 t3 2
      add t4 a0 t4
      lw t5 0(t4)

      ble t5 t2 .Linsertion_sort_insert

      # a[j+1] = a[j]
      sw t5 4(t4)

      dec t3
      j .Linsertion_sort_inner

    .Linsertion_sort_insert:
      # a[j+1] = t2
      inc t3
      sll t3 t3 2
      add t3 a0 t3
      sw t2 0(t3)

      inc t0
      j .Linsertion_sort_outer

  .Linsertion_sort_done:
    ret

read_num_with_seg_disp:
  sub sp sp 4
  sw s0 0(sp)

  clr s0

  .Lread_digit_with_seg_disp_loop:
    seg s0
    call read_digit
    beqz a0 .Lread_digit_with_seg_disp_loop

  mv a0 s0

  lw s0 0(sp)
  add sp sp 4

  ret

# s0 = current accumulated value
# a0 = done flag (0 / 1)
read_digit:
  call read_key  # a0 = keycode

  # ENTER
  beq a0 KEY_ENTER .Lread_digit_enter

  # KEY_0
  beq a0 KEY_0 .Lread_digit_is_zero

  # digit = key - KEY_1 + 1
  sub t0 a0 KEY_1
  inc t0
  j .Lread_digit_acc

  .Lread_digit_is_zero:
    li t0 0

  .Lread_digit_acc:
    # s0 = s0 * 10 + digit
    mul s0 s0 10
    add s0 s0 t0

    li a0 0
    ret

  .Lread_digit_enter:
    li a0 1
    ret

read_key:
  mv a0 kb

  # a0 not in [KEY_1, KEY_0] or not KEY_ENTER
  blt a0 KEY_1 read_key
  beq a0 KEY_ENTER .Lread_key_end
  bgt a0 KEY_0 read_key

  .Lread_key_end: ret

wait:
  mv t0 kb
  beqz t0 wait
  ret

main:
  call read_num_with_seg_disp
  mv a1 a0  # length

  # malloc from stack frame
  sll s0 a1 2  # size = length * 4
  sub sp sp s0

  mv t3 a1
  mv t1 sp

  .Lmain_input_loop:
    beqz t3 .Lmain_input_done

    call read_num_with_seg_disp
    mv t2 a0

    sw t2 0(t1)
    add t1 t1 4

    dec t3
    j .Lmain_input_loop

  .Lmain_input_done:
    segi 255  # just a symbolic way to indicate "input done"
    call wait
    mv a0 sp
    call insertion_sort

  segi 254  # just indicate "sorting done"
  call wait

  .Lmain_output_loop:
    beq a1 zero .Lmain_output_done

    lw t0 0(a0)
    seg t0
    call wait
    add a0 a0 4

    dec a1
    j .Lmain_output_loop

  .Lmain_output_done:
    add sp sp s0
    segi 0
    j halt

halt:
  j halt
