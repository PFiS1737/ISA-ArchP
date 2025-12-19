# leetcode: trapping-rain-water
# solutions/5126477

const l r1
const r r2
const lmax r3
const rmax r4
const water r5

const t0 r6
const t1 r7

const a0 r8
const a1 r9
const rt r10

main:
  li t0 16
  input:
    sw io t1
    inc t1
    dec t0
    bgtz t0 input

  call solve

  mv io rt

solve:
  # l = 0
  li l 0
  # r = len - 1 = 15
  li r 15
  # lmax = ht[l]
  lw lmax l
  # rmax = ht[r]
  lw rmax r

  # while l < r:
  while: bge l r endwhile
    # if lmax < rmax:
    bge lmax rmax else
      # l++
      inc l
      # lmax = max(lmax, ht[l])
      lw a0 l
      mv a1 lmax
      call max
      mv lmax rt
      # water += lmax - ht[l]
      sub t1 lmax a0
      add water water t1

      jmp endif

    else:
      # r--
      dec r
      # rmax = max(rmax, ht[r])
      lw a0 r
      mv a1 rmax
      call max
      mv rmax rt
      # water += rmax - ht[r]
      sub t1 rmax a0
      add water water t1

    endif:
      jmp while

  endwhile:
    mv rt water
    ret

max:
  cmp a0 a1
  mv.gt rt a0
  mv.le rt a1
  ret
