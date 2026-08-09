.const head_x, r1
.const head_y, r2
.const food_x, r3
.const food_y, r4
.const t0, r5
.const t1, r6
.const nx, r7
.const ny, r8
.const key_code, r9
.const x, r10 ; a0
.const y, r11 ; a1
.const color, r12 ; a2
.const i, r13
.const rt, r14
.const que_len, r15
.const que_head, r16
; r17 is used for syscalls
.const que_val, r18
.const que_tmp, r19
.const que_i, r20
.const que_cur, r21
.const que_addr, r22

.const BASE_ADDR, 0x00100000
.const KBD_BASE, 0x90000000

.const SCREEN_WIDTH, 64
.const SCREEN_HEIGHT, 36

.const MAX_X, 63 # SCREEN_WIDTH - 1
.const MAX_Y, 35 # SCREEN_HEIGHT - 1

.const COLOR_BACK, 0x000000
.const COLOR_BODY, 0xFFFFFF
.const COLOR_HEAD, 0xFF0000
.const COLOR_FOOD, 0xFFFF00

# linux input-event-codes
.const KEY_UP, 103
.const KEY_DOWN, 108
.const KEY_LEFT, 105
.const KEY_RIGHT, 106

.const INIT_X, 5
.const INIT_Y, 18

.const QUEUE_SIZE, 100


j main


init_screen:
  li x, 0
  li y, 0
  li color, COLOR_BACK
  draw_back:
    ecall 0x10000000 ; set pixel
    inc x
    blt x, SCREEN_WIDTH, draw_back
    inc y
    li x, 0
    blt y, SCREEN_HEIGHT, draw_back

  ret


init_snake:
  li color, COLOR_HEAD
  li head_x, INIT_X
  li head_y, INIT_Y

  mv x, head_x
  mv y, head_y
  ecall 0x10000000 ; set pixel

  li color, COLOR_BODY
  li i, 3
  init_body_loop:
    sub x, head_x, i
    mv y, head_y
    call body_push
    ecall 0x10000000 ; set pixel
    dec i
    bge i, 1, init_body_loop

  ret


move_snake:
  beq key_code, KEY_UP, move_snake_up
  beq key_code, KEY_DOWN, move_snake_down
  beq key_code, KEY_LEFT, move_snake_left
  beq key_code, KEY_RIGHT, move_snake_right

  ret

  move_snake_up:
    mv nx, head_x
    sub ny, head_y, 1
    j move_snake_common

  move_snake_down:
    mv nx, head_x
    add ny, head_y, 1
    j move_snake_common

  move_snake_left:
    sub nx, head_x, 1
    mv ny, head_y
    j move_snake_common

  move_snake_right:
    add nx, head_x, 1
    mv ny, head_y
    j move_snake_common

  move_snake_common:
    # 旧头变成身体
    mv x, head_x
    mv y, head_y
    call body_push
    li color, COLOR_BODY
    ecall 0x10000000 ; set pixel

    # 允许环绕
    bge nx, 0, move_snake_common_x_ge_0
    li nx, MAX_X
    move_snake_common_x_ge_0:

    blt nx, SCREEN_WIDTH, move_snake_common_x_lt_width
    li nx, 0
    move_snake_common_x_lt_width:

    bge ny, 0, move_snake_common_y_ge_0
    li ny, MAX_Y
    move_snake_common_y_ge_0:

    blt ny, SCREEN_HEIGHT, move_snake_common_y_lt_height
    li ny, 0
    move_snake_common_y_lt_height:

    # 画新头
    mv head_x, nx
    mv head_y, ny
    li color, COLOR_HEAD

    mv x, head_x
    mv y, head_y
    ecall 0x10000000 ; set pixel

    # 检查是否撞到自己
    call body_contains
    beq rt, 1, lose_loop

    # 吃到食物则不动尾巴，重新生成食物
    bne head_x, food_x, move_snake_not_eat
    bne head_y, food_y, move_snake_not_eat
    call gen_food
    ret

    # 未吃到则弹出尾巴
    move_snake_not_eat:
      call body_pop
      li color, COLOR_BACK
      ecall 0x10000000 ; set pixel
      ret


gen_food:
  ecall 41 ; random int
  mv y, x
  ecall 41
  rem x, x, SCREEN_WIDTH
  rem y, y, SCREEN_HEIGHT

  call body_contains
  beq rt, 1, gen_food

  mv food_x, x
  mv food_y, y
  li color, COLOR_FOOD
  ecall 0x10000000 ; set pixel

  ret


body_push:
  sll que_val, x, 8
  or que_val, que_val, y
  call queue_push
  ret

body_pop:
  call queue_pop
  srl x, que_val, 8
  and y, que_val, 0x0FF
  ret

body_contains:
  sll que_val, x, 8
  or que_val, que_val, y
  call queue_contains
  ret


# if (len == SIZE) return 1
# queue[head] = val
# head = (head + 1) % SIZE
# len++
# return 0
queue_push:
  beq que_len, QUEUE_SIZE, queue_push_full

  mul que_tmp, que_head, 2
  add que_tmp, que_tmp, BASE_ADDR
  sh que_val, (que_tmp)

  inc que_head
  rem que_head, que_head, QUEUE_SIZE

  inc que_len

  li rt, 0
  ret

  queue_push_full:
    li rt, 1
    ret

# if (len == 0) return 1
# t = (head - len + SIZE) % SIZE
# val = queue[t]
# len--
# return 0
queue_pop:
  beqz que_len, queue_pop_empty

  sub que_tmp, que_head, que_len
  add que_tmp, que_tmp, QUEUE_SIZE
  rem que_tmp, que_tmp, QUEUE_SIZE

  mul que_tmp, que_tmp, 2
  add que_tmp, que_tmp, BASE_ADDR
  lhu que_val, (que_tmp)

  dec que_len

  li rt, 0
  ret

  queue_pop_empty:
    li rt, 1
    ret

# if (len == 0) return 0
# for i in [0, len):
#   idx = (head - len + i + SIZE) % SIZE
#   if queue[idx] == val: return 1
# return 0
queue_contains:
  beqz que_len, queue_contains_not_found

  # que_tmp = (head - len + SIZE) % SIZE
  sub que_tmp, que_head, que_len
  add que_tmp, que_tmp, QUEUE_SIZE
  rem que_tmp, que_tmp, QUEUE_SIZE

  li que_i, 0

  queue_contains_loop:
    beq que_i, que_len, queue_contains_not_found

    mul que_addr, que_tmp, 2
    add que_addr, que_addr, BASE_ADDR
    lhu que_cur, (que_addr)

    beq que_cur, que_val, queue_contains_found

    inc que_tmp
    rem que_tmp, que_tmp, QUEUE_SIZE

    inc que_i
    j queue_contains_loop

  queue_contains_found:
    li rt, 1
    ret

  queue_contains_not_found:
    li rt, 0
    ret


read_key:
  li t0, KBD_BASE
  lw t0, (t0)

  beq t0, KEY_UP, read_key_ok
  beq t0, KEY_DOWN, read_key_ok
  beq t0, KEY_LEFT, read_key_ok
  beq t0, KEY_RIGHT, read_key_ok

  ret

  read_key_ok:
    mv key_code, t0

  read_key_ret:
    ret


main:
  call init_screen
  call init_snake
  call gen_food

  main_loop:
    call read_key
    call move_snake
    j main_loop


lose_loop:
  j lose_loop
