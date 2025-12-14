const head_x r1
const head_y r2
const food_x r3
const food_y r4
const x r5
const y r6
const nx r7
const ny r8
const key_code r9
const t0 r10
const t1 r11
const i r12

const rt r13

const que_len r14
const que_head r15
const que_val r16
const que_tmp r17
const que_i r18
const que_cur r19

const SCREEN_WIDTH 64
const SCREEN_HEIGHT 36

const MAX_X 63 # SCREEN_WIDTH - 1
const MAX_Y 35 # SCREEN_HEIGHT - 1

const COLOR_BACK 0x000000
const COLOR_BODY 0xFFFFFF
const COLOR_HEAD 0xFF0000
const COLOR_FOOD 0xFFFF00

const KEY_UP 70
const KEY_DOWN 72
const KEY_LEFT 69
const KEY_RIGHT 71

const INIT_X 5
const INIT_Y 18

const QUEUE_SIZE 100


jmp main


init_screen:
	clr x
	clr y
	col COLOR_BACK
	draw_back:
		spx x y
		inc x
		blt x SCREEN_WIDTH draw_back
		inc y
		clr x
		blt y SCREEN_HEIGHT draw_back

  ret


init_snake:
  col COLOR_HEAD
  li head_x INIT_X
  li head_y INIT_Y
  spx head_x head_y

  col COLOR_BODY
  li i 3
  init_body_loop:
    sub x head_x i
    mv y head_y
    call body_push
    spx x y
    dec i
    bge i 1 init_body_loop

  ret


move_snake:
  beq key_code KEY_UP move_snake_up
  beq key_code KEY_DOWN move_snake_down
  beq key_code KEY_LEFT move_snake_left
  beq key_code KEY_RIGHT move_snake_right

  ret

  move_snake_up:
    mv nx head_x
    sub ny head_y 1
    jmp move_snake_common

  move_snake_down:
    mv nx head_x
    add ny head_y 1
    jmp move_snake_common

  move_snake_left:
    sub nx head_x 1
    mv ny head_y
    jmp move_snake_common

  move_snake_right:
    add nx head_x 1
    mv ny head_y
    jmp move_snake_common

  move_snake_common:
    # 旧头变成身体
    mv x head_x
    mv y head_y
    call body_push
    col COLOR_BODY
    spx x y

    # 允许环绕
    cmp nx 0
    li.lt nx MAX_X
    cmp nx SCREEN_WIDTH
    li.ge nx 0
    cmp ny 0
    li.lt ny MAX_Y
    cmp ny SCREEN_HEIGHT
    li.ge ny 0

    # 画新头
    mv head_x nx
    mv head_y ny
    col COLOR_HEAD
    spx head_x head_y

    # 检查是否撞到自己
    mv x head_x
    mv y head_y
    call body_contains
    beq rt 1 lose_loop

    # 吃到食物则不动尾巴，重新生成食物
    bne head_x food_x move_snake_not_eat
    bne head_y food_y move_snake_not_eat
    call gen_food
    ret

    # 未吃到则弹出尾巴
    move_snake_not_eat:
      call body_pop
      col COLOR_BACK
      spx x y
      ret


gen_food:
  mv x rng
  mv y rng
  mod x x SCREEN_WIDTH
  mod y y SCREEN_HEIGHT

  call body_contains
  beq rt 1 gen_food

  mv food_x x
  mv food_y y
  col COLOR_FOOD
  spx food_x food_y

  ret


body_push:
  shl que_val x 8
  or que_val que_val y
  call queue_push
  ret

body_pop:
  call queue_pop
  shr x que_val 8
  and y que_val 0xFF
  ret

body_contains:
  shl que_val x 8
  or que_val que_val y
  call queue_contains
  ret


# if (len == SIZE) return 1
# queue[head] = val
# head = (head + 1) % SIZE
# len++
# return 0
queue_push:
  beq que_len QUEUE_SIZE queue_push_full

  sw que_head que_val 0

  inc que_head
  mod que_head que_head QUEUE_SIZE

  inc que_len

  li rt 0
  ret

  queue_push_full:
    li rt 1
    ret

# if (len == 0) return 1
# t = (head - len + SIZE) % SIZE
# val = queue[t]
# len--
# return 0
queue_pop:
  beq que_len 0 queue_pop_empty

  sub que_tmp que_head que_len
  add que_tmp que_tmp QUEUE_SIZE
  mod que_tmp que_tmp QUEUE_SIZE

  lw que_val que_tmp 0

  dec que_len

  li rt 0
  ret

  queue_pop_empty:
    li rt 1
    ret

# if (len == 0) return 0
# for i in [0, len):
#   idx = (head - len + i + SIZE) % SIZE
#   if queue[idx] == val: return 1
# return 0
queue_contains:
  beq que_len 0 queue_contains_not_found

  # que_tmp = (head - len + SIZE) % SIZE
  sub que_tmp que_head que_len
  add que_tmp que_tmp QUEUE_SIZE
  mod que_tmp que_tmp QUEUE_SIZE

  clr que_i

  queue_contains_loop:
    beq que_i que_len queue_contains_not_found

    lw que_cur que_tmp 0
    beq que_cur que_val queue_contains_found

    inc que_tmp
    mod que_tmp que_tmp QUEUE_SIZE

    inc que_i
    jmp queue_contains_loop

  queue_contains_found:
    li rt 1
    ret

  queue_contains_not_found:
    li rt 0
    ret


read_key:
  mv t0 kb

  beq t0 KEY_UP read_key_ok
  beq t0 KEY_DOWN read_key_ok
  beq t0 KEY_LEFT read_key_ok
  beq t0 KEY_RIGHT read_key_ok

  ret

  read_key_ok:
    sub t1 key_code t0
    beq t1 2 read_key_ret
    beq t1 0xFFFFFFFE read_key_ret # -2

    mv key_code t0

  read_key_ret:
    ret


main:
  call init_screen
  call init_snake
  call gen_food

  main_loop:
    li i 100
    sleep:
      dec i
      bgt i 0 sleep

    call read_key
    call move_snake
    jmp main_loop


lose_loop:
  jmp lose_loop
