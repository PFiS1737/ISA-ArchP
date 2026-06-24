# leetcode: trapping-rain-water
# solutions/5126477

const l r1
const r r2
const lmax r3
const rmax r4
const water r5

const t0 r6
const t1 r7
const t3 r8

const a0 r9
const a1 r10

main:
  li t0 16
  input:
    in t3
    sb t3 t1
    inc t1
    dec t0
    bgtz t0 input

  call solve

  out a0

solve:
  # l = 0
  li l 0
  # r = len - 1 = 15
  li r 15
  # lmax = ht[l]
  lbu lmax l
  # rmax = ht[r]
  lbu rmax r

  # while l < r:
  while: bge l r endwhile
    # if lmax < rmax:
    bge lmax rmax else
      # l++
      inc l
      # lmax = max(lmax, ht[l])
      lbu t0 l
      mv a0 t0
      mv a1 lmax
      call max
      mv lmax a0
      # water += lmax - ht[l]
      sub t1 lmax t0
      add water water t1

      j endif

    else:
      # r--
      dec r
      # rmax = max(rmax, ht[r])
      lbu t0 r
      mv a0 t0
      mv a1 rmax
      call max
      mv rmax a0
      # water += rmax - ht[r]
      sub t1 rmax t0
      add water water t1

    endif:
      j while

  endwhile:
    mv a0 water
    ret

max:
  bgt a0 a1 max_ret
  mv a0 a1
  max_ret:
  ret
