# ref https://github.com/ESnake37/Turing-Complete-Minesweeper/blob/main/MINESWEEPER.asm

const x r1
const y r2
const i r3
const j r4
const t0 r5
const t1 r6
const t2 r7
const cursor_x r8
const cursor_y r9
const addr r10
const tx r11
const ty r12
const nx r13
const ny r14
const cnt r15
const key_code r16
const arg_x r17
const arg_y r18
const mine_num r19

const BASE_ADDR 0x00100000
const KBD_BASE 0x90000000

const SCREEN_WIDTH 128
const SCREEN_HEIGHT 72

const GRID_WIDTH 121
const GRID_HEIGHT 65

const GRID_COLS 15
const GRID_ROWS 8
const TILE_SIZE 8

const COLOR_BACK 0x181A1BFF
const COLOR_HIDDEN 0x4C545CFF
const COLOR_REVEALED 0x384048FF
const COLOR_GRID_LINE 0x22262EFF
const COLOR_CURSOR 0xD6BB15FF
const COLOR_MINE 0x000000FF
const COLOR_MINE_BACK 0xEE6666FF
const COLOR_FLAG 0xF75050FF
const COLOR_POLE 0xD8E0E8FF
const COLOR_NUM1 0x7CC7ffFF
const COLOR_NUM2 0x66C266FF
const COLOR_NUM3 0xFF7788FF
const COLOR_NUM4 0xEE88FFFF
const COLOR_NUM5 0xDDAA22FF

const MINE_NUM_MAX 16

const AROUND_COUNT_MASK 7
const MINE_MASK 8
const REVEAL_MASK 16
const FLAG_MASK 32

# linux input-event-codes
const KEY_UP 103
const KEY_DOWN 108
const KEY_LEFT 105
const KEY_RIGHT 106
const KEY_REVEAL 44 # 'z'
const KEY_FLAG 45 # 'x'


j main


init_screen:
  # 画背景
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

  # 画格子
  clr x
  clr y
  col COLOR_HIDDEN
  draw_grid:
    spx x y
    inc x
    blt x GRID_WIDTH draw_grid
    inc y
    clr x
    blt y GRID_HEIGHT draw_grid

  # 画分隔线
  clr x
  clr y
  col COLOR_GRID_LINE
  draw_grid_line_row:
    spx x y
    inc x
    blt x GRID_WIDTH draw_grid_line_row
    add y y TILE_SIZE
    clr x
    blt y GRID_HEIGHT draw_grid_line_row
  clr x
  clr y
  draw_grid_line_col:
    spx x y
    inc y
    blt y GRID_HEIGHT draw_grid_line_col
    add x x TILE_SIZE
    clr y
    blt x GRID_WIDTH draw_grid_line_col

  # 画光标
  clr cursor_x
  clr cursor_y
  col COLOR_CURSOR
  call update_cursor

  ret


init_mines:
  li mine_num MINE_NUM_MAX
  out mine_num
  clr i

  init_mines_loop:
    rand t1
    rand t2
    rem x t1 GRID_COLS
    rem y t2 GRID_ROWS

    mul addr y GRID_COLS
    add addr addr x
    mul addr addr 4
    add addr addr BASE_ADDR

    lw t1 addr
    beq t1 MINE_MASK init_mines_loop

    li t1 MINE_MASK
    sw t1 addr
    inc i
    blt i mine_num init_mines_loop

  ret


init_mine_counts:
  clr x
  clr y
  li addr BASE_ADDR
  init_mine_counts_loop:
    call count_around_mines
    lw t1 addr
    add t2 t1 cnt
    sw t2 addr
    inc x
    add addr addr 4
    blt x GRID_COLS init_mine_counts_loop
    clr x
    inc y
    blt y GRID_ROWS init_mine_counts_loop

  ret


count_around_mines:
  clr cnt
  li ty -1

  dy_loop:
    li tx -1

  dx_loop:
    add nx x tx
    add ny y ty

    # 越界判断
    bltz nx skip_this_neighbor
    bge nx GRID_COLS skip_this_neighbor
    bltz ny skip_this_neighbor
    bge ny GRID_ROWS skip_this_neighbor

    # 计算地址
    mul t1 ny GRID_COLS
    add t1 t1 nx
    mul t1 t1 4
    add t1 t1 BASE_ADDR

    # 读取格子值
    lw t2 t1

    # 判断是否为地雷
    and t2 t2 MINE_MASK
    bne t2 MINE_MASK skip_this_neighbor
    inc cnt

  skip_this_neighbor:
    inc tx
    ble tx 1 dx_loop
    inc ty
    ble ty 1 dy_loop

  ret


move_cursor:
  col COLOR_GRID_LINE
  call update_cursor

  bne key_code KEY_UP move_cursor_key_up_ne
  dec cursor_y
  move_cursor_key_up_ne:

  bne key_code KEY_DOWN move_cursor_key_down_ne
  inc cursor_y
  move_cursor_key_down_ne:

  bne key_code KEY_LEFT move_cursor_key_left_ne
  dec cursor_x
  move_cursor_key_left_ne:

  bne key_code KEY_RIGHT move_cursor_key_right_ne
  inc cursor_x
  move_cursor_key_right_ne:

  # ---------- 回绕 ----------

  bge cursor_x 0 move_cursor_x_ge_0
  li cursor_x 14
  move_cursor_x_ge_0:

  blt cursor_x GRID_COLS move_cursor_x_lt_cols
  li cursor_x 0
  move_cursor_x_lt_cols:

  bge cursor_y 0 move_cursor_y_ge_0
  li cursor_y 7
  move_cursor_y_ge_0:

  blt cursor_y GRID_ROWS move_cursor_y_lt_rows
  li cursor_y 0
  move_cursor_y_lt_rows:

  col COLOR_CURSOR
  call update_cursor

  ret

update_cursor:
  clr i
  clr j

  mul x cursor_x TILE_SIZE
  mul y cursor_y TILE_SIZE

  update_cursor_loop1:
    spx x y
    inc x
    inc i
    ble i TILE_SIZE update_cursor_loop1
    mul x cursor_x TILE_SIZE
    clr i
    add y y TILE_SIZE
    inc j
    beq j 1 update_cursor_loop1

  mul x cursor_x TILE_SIZE
  mul y cursor_y TILE_SIZE

  update_cursor_loop2:
    spx x y
    inc y
    inc i
    ble i TILE_SIZE update_cursor_loop2
    mul y cursor_y TILE_SIZE
    clr i
    add x x TILE_SIZE
    inc j
    beq j 3 update_cursor_loop2

  ret


reveal_tile:
  mul addr cursor_y GRID_COLS
  add addr addr cursor_x
  mul addr addr 4
  add addr addr BASE_ADDR

  lw t1 addr

  and t2 t1 REVEAL_MASK
  bnez t2 reveal_tile_ret
  and t2 t1 FLAG_MASK
  bnez t2 reveal_tile_ret
  and t2 t1 MINE_MASK
  bnez t2 lose_loop
  and t2 t1 AROUND_COUNT_MASK

  mv arg_x cursor_x
  mv arg_y cursor_y

  bne t2 0 reveal_tile_skip_reveal_around
  call reveal_around
  reveal_tile_skip_reveal_around:

  or t1 t1 REVEAL_MASK
  sw t1 addr

  call draw_num

  reveal_tile_ret:
    ret


reveal_around:
  # 初始化当前坐标
  mv nx cursor_x
  mv ny cursor_y

  reveal_around_loop:
    # 越界检查
    bltz nx reveal_around_ret
    bge nx GRID_COLS reveal_around_ret
    bltz ny reveal_around_ret
    bge ny GRID_ROWS reveal_around_ret

    # 计算地址
    mul addr ny GRID_COLS
    add addr addr nx
    mul addr addr 4
    add addr addr BASE_ADDR

    # 读取格子值
    lw t1 addr

    # 如果已揭示或有旗子，则返回
    and t2 t1 REVEAL_MASK
    bnez t2 reveal_around_ret
    and t2 t1 FLAG_MASK
    bnez t2 reveal_around_ret

    # 设置为已揭示
    or t1 t1 REVEAL_MASK
    sw t1 addr

    # 获取周围雷数
    and t2 t1 AROUND_COUNT_MASK

    # 根据雷数绘图
    mv arg_x nx
    mv arg_y ny

    bne t2 0 reveal_around_skip_draw_tile
    col COLOR_REVEALED
    call draw_tile
    reveal_around_skip_draw_tile:

    call draw_num

    # 如果不是空白格，不再递归
    bnez t2 reveal_around_ret

    # 递归
    dec ny
    call reveal_around_loop
    inc ny

    inc ny
    call reveal_around_loop
    dec ny

    dec nx
    call reveal_around_loop
    inc nx

    inc nx
    call reveal_around_loop
    dec nx

  reveal_around_ret:
    ret


toggle_flag:
  mul addr cursor_y GRID_COLS
  add addr addr cursor_x
  mul addr addr 4
  add addr addr BASE_ADDR

  lw t1 addr

  and t2 t1 REVEAL_MASK
  bnez t2 toggle_flag_ret

  mv arg_x cursor_x
  mv arg_y cursor_y

  and t2 t1 FLAG_MASK
  bnez t2 toggle_flag_already_set

  dec mine_num
  or t1 t1 FLAG_MASK
  call draw_flag
  j toggle_flag_store

  toggle_flag_already_set:
    inc mine_num
    li t2 -33 ; not t2 FLAG_MASK
    and t1 t1 t2
    col COLOR_HIDDEN
    call draw_tile

  toggle_flag_store:
    sw t1 addr
    out mine_num

  toggle_flag_ret:
    ret


draw_tile:
  clr i
  clr j
  mul x arg_x TILE_SIZE
  mul y arg_y TILE_SIZE
  inc x
  inc y
  draw_tile_loop:
    spx x y
    inc x
    inc i
    blt i 7 draw_tile_loop
    mul x arg_x TILE_SIZE
    inc x
    inc y
    clr i
    inc j
    blt j 7 draw_tile_loop

  ret


draw_flag:
  mul x arg_x TILE_SIZE
  mul y arg_y TILE_SIZE
  col COLOR_FLAG
  add x x 4
  add y y 2
  spx x y
  inc y
  spx x y
  inc y
  spx x y
  dec x
  spx x y
  dec x
  spx x y
  inc x
  dec y
  spx x y
  inc x
  add y y 2
  spx x y
  sub x x 2
  inc y
  col COLOR_POLE
  clr i
  draw_pole:
    spx x y
    inc x
    inc i
    blt i 5 draw_pole

  ret


draw_mine:
  mul x arg_x TILE_SIZE
  mul y arg_y TILE_SIZE
  add x x 4
  add y y 2
  col COLOR_MINE
  spx x y
  inc y
  spx x y
  dec x
  spx x y
  add x x 2
  spx x y
  inc y
  spx x y
  inc x
  spx x y
  sub x x 2
  spx x y
  dec x
  spx x y
  dec x
  spx x y
  inc x
  inc y
  spx x y
  inc x
  spx x y
  inc x
  spx x y
  dec x
  inc y
  spx x y

  ret

draw_num:
  # @t2: number to draw (1-5)

  bne t2 1 skip_draw1
  call draw_num1
  skip_draw1:

  bne t2 2 skip_draw2
  call draw_num2
  skip_draw2:

  bne t2 3 skip_draw3
  call draw_num3
  skip_draw3:

  bne t2 4 skip_draw4
  call draw_num4
  skip_draw4:

  bne t2 5 skip_draw5
  call draw_num5
  skip_draw5:

  ret

draw_num1:
  col COLOR_REVEALED
  call draw_tile
  clr i
  clr j
  mul x arg_x TILE_SIZE
  mul y arg_y TILE_SIZE
  add x x 4
  add y y 2
  col COLOR_NUM1
  draw_num1_loop:
    spx x y
    inc y
    inc i
    blt i 5 draw_num1_loop

  ret


draw_num2:
  col COLOR_REVEALED
  call draw_tile
  clr i
  clr j
  mul x arg_x TILE_SIZE
  mul y arg_y TILE_SIZE
  add x x 3
  add y y 2
  col COLOR_NUM2
  draw_num2_loop:
    spx x y
    inc x
    inc i
    blt i 3 draw_num2_loop
    sub x x 3
    add y y 2
    clr i
    inc j
    blt j 3 draw_num2_loop
    sub y y 3
    spx x y
    add x x 2
    sub y y 2
    spx x y

  ret


draw_num3:
  col COLOR_REVEALED
  call draw_tile
  clr i
  clr j
  mul x arg_x TILE_SIZE
  mul y arg_y TILE_SIZE
  add x x 3
  add y y 2
  col COLOR_NUM3
  draw_num3_loop:
    spx x y
    inc x
    inc i
    blt i 3 draw_num3_loop
    sub x x 3
    add y y 2
    clr i
    inc j
    blt j 3 draw_num3_loop
    add x x 2
    sub y y 3
    spx x y
    sub y y 2
    spx x y

  ret


draw_num4:
  col COLOR_REVEALED
  call draw_tile
  clr i
  mul x arg_x TILE_SIZE
  mul y arg_y TILE_SIZE
  add x x 3
  add y y 2
  col COLOR_NUM4
  draw_num4_loop1:
    spx x y
    inc y
    inc i
    blt i 3 draw_num4_loop1
  inc x
  dec y
  spx x y
  inc x
  sub y y 2
  draw_num4_loop2:
    spx x y
    inc y
    inc i
    blt i 8 draw_num4_loop2

  ret


draw_num5:
  col COLOR_REVEALED
  call draw_tile
  clr i
  clr j
  mul x arg_x TILE_SIZE
  mul y arg_y TILE_SIZE
  add x x 3
  add y y 2
  col COLOR_NUM5
  draw_num5_loop:
    spx x y
    inc x
    inc i
    blt i 3 draw_num5_loop
    sub x x 3
    add y y 2
    clr i
    inc j
    blt j 3 draw_num5_loop
    sub y y 5
    spx x y
    add x x 2
    add y y 2
    spx x y

  ret


read_key:
  li t0 KBD_BASE
  lw key_code t0
  beqz key_code read_key
  read_key_wait_release:
    lw t1 t0
    bnez t1 read_key_wait_release
  ret


main:
  call init_screen
  call init_mines
  call init_mine_counts

  main_loop:
    call read_key

    beq key_code KEY_UP main_loop_move_cursor
    beq key_code KEY_DOWN main_loop_move_cursor
    beq key_code KEY_LEFT main_loop_move_cursor
    beq key_code KEY_RIGHT main_loop_move_cursor
    beq key_code KEY_REVEAL main_loop_reveal_tile
    beq key_code KEY_FLAG main_loop_toggle_flag

    main_loop_move_cursor:
      call move_cursor
      j main_loop
    main_loop_reveal_tile:
      call reveal_tile
      j main_loop
    main_loop_toggle_flag:
      call toggle_flag
      j main_loop


  win_loop:
    j win_loop

  lose_loop:
    mv arg_x cursor_x
    mv arg_y cursor_y
    col COLOR_MINE_BACK
    call draw_tile
    call draw_mine

    clr ny
    li addr BASE_ADDR
    lose_row_loop:
      clr nx
    lose_col_loop:
      bne nx cursor_x lose_loop_cont
      bne ny cursor_y lose_loop_cont
      j lose_loop_skip_draw
    lose_loop_cont:
      lw t1 addr
      and t2 t1 MINE_MASK
      beqz t2 lose_loop_skip_draw
      mv arg_x nx
      mv arg_y ny
      col COLOR_REVEALED
      call draw_tile
      call draw_mine
    lose_loop_skip_draw:
      inc nx
      add addr addr 4
      blt nx GRID_COLS lose_col_loop
      inc ny
      blt ny GRID_ROWS lose_row_loop

halt:
  j halt
