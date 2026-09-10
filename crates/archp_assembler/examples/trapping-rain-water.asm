# leetcode: trapping-rain-water
# solutions/5126477

.alias len, s0
.alias l, s1
.alias r, s2
.alias lmax, s3
.alias rmax, s4
.alias water, s5

.equ BASE_ADDR, 0x00100000

li sp, 0x000FFFFF

main:
  ecall 5 ; read int
  mv len, a0

  mv t0, len
  li t1, BASE_ADDR
  input:
    ecall 5 ; read int
    sb a0, (t1)
    inc t1
    dec t0
    bgtz t0, input

  call solve

  ecall 1 ; print int

  ecall 10 ; exit

solve:
  sub sp, sp, 4
  sw ra, 0(sp)
  # l = 0
  li l, BASE_ADDR
  # r = len - 1
  add r, l, len
  dec r
  # lmax = ht[l]
  lbu lmax, (l)
  # rmax = ht[r]
  lbu rmax, (r)

  # while l < r:
  while: bge l, r, endwhile
    # if lmax < rmax:
    bge lmax, rmax, else
      # l++
      inc l
      # lmax = max(lmax, ht[l])
      lbu t0, (l)
      mv a0, t0
      mv a1, lmax
      call max
      mv lmax, a0
      # water += lmax - ht[l]
      sub t1, lmax, t0
      add water, water, t1

      j endif

    else:
      # r--
      dec r
      # rmax = max(rmax, ht[r])
      lbu t0, (r)
      mv a0, t0
      mv a1, rmax
      call max
      mv rmax, a0
      # water += rmax - ht[r]
      sub t1, rmax, t0
      add water, water, t1

    endif:
      j while

  endwhile:
    mv a0, water
    lw ra, 0(sp)
    add sp, sp, 4
    ret

max:
  bgt a0, a1, max_ret
  mv a0, a1
  max_ret:
  ret
