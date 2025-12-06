const s0 r1
const s1 r2
const s2 r3

const a0 r4
const rt r5

main:
  li a0 10
  call fib
  mv io rt

fib:
  mv s0 a0

  cmpi s0 2
  li.le rt 1
  ret.le

  # fib(n - 1)
  push s0
  push s1
  push s2
  subi s0 s0 1
  mv a0 s0
  call fib
  pop s2
  pop s1
  pop s0
  mv s1 rt

  # fib(n - 2)
  push s0
  push s1
  push s2
  subi s0 s0 2
  mv a0 s0
  call fib
  pop s2
  pop s1
  pop s0
  mv s2 rt

  add rt s1 s2

  ret
