# ref https://github.com/ESnake37/Turing-Complete-Minesweeper/blob/main/MINESWEEPER.asm

const x r1
const y r2
const i r3
const j r4
const t1 r5
const t2 r6
const cursor_x r7
const cursor_y r8
const addr r9
const tx r10
const ty r11
const nx r12
const ny r13
const cnt r14
const key_code r15
const arg_x r16
const arg_y r17
const mine_num r18

const SCREEN_WIDTH 128
const SCREEN_HEIGHT 72

const GRID_WIDTH 121
const GRID_HEIGHT 65

const GRID_COLS 15
const GRID_ROWS 8
const TILE_SIZE 8

const COLOR_BACK 0x181A1B
const COLOR_HIDDEN 0x4C545C
const COLOR_REVEALED 0x384048
const COLOR_GRID_LINE 0x22262E
const COLOR_CURSOR 0xD6BB15
const COLOR_MINE 0x000000
const COLOR_MINE_BACK 0xEE6666
const COLOR_FLAG 0xF75050
const COLOR_POLE 0xD8E0E8
const COLOR_NUM1 0x7CC7ff
const COLOR_NUM2 0x66C266
const COLOR_NUM3 0xFF7788
const COLOR_NUM4 0xEE88FF
const COLOR_NUM5 0xDDAA22

const MINE_NUM_MAX 16

const AROUND_COUNT_MASK 7
const MINE_MASK 8
const REVEAL_MASK 16
const FLAG_MASK 32

const KEY_UP 70
const KEY_DOWN 72
const KEY_LEFT 69
const KEY_RIGHT 71
const KEY_REVEAL 55 # 'z'
const KEY_FLAG 56 # 'x'


jmp main


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
	clr i

	init_mines_loop:
		mv t1 rng
		mv t2 rng
		mod x t1 GRID_COLS
		mod y t2 GRID_ROWS
		mull addr y GRID_COLS
		add addr addr x
		lw t1 addr 0
		beq t1 MINE_MASK init_mines_loop

    li t1 MINE_MASK
		sw addr t1 0
		inc i
		blt i mine_num init_mines_loop

	ret


init_mine_counts:
	clr x
	clr y
	clr addr
	init_mine_counts_loop:
		call count_around_mines
		lw t1 addr 0
		add t2 t1 cnt
		sw addr t2 0
		inc x
		inc addr
		blt x GRID_COLS init_mine_counts_loop
		clr x
		inc y
		blt y GRID_ROWS init_mine_counts_loop

	ret


count_around_mines:
	clr cnt
	li ty 0xFFFFFFFF # -1

	dy_loop:
		li tx 0xFFFFFFFF # -1

	dx_loop:
		add nx x tx
		add ny y ty

		# 越界判断
		blt nx 0 skip_this_neighbor
		bge nx GRID_COLS skip_this_neighbor
		blt ny 0 skip_this_neighbor
		bge ny GRID_ROWS skip_this_neighbor

		# 计算地址
		mull t1 ny GRID_COLS
		add t1 t1 nx

		# 读取格子值
		lw t2 t1 0
		
		# 判断是否为地雷
		and t2 t2 MINE_MASK
		cmp t2 MINE_MASK
		inc.eq cnt
	
	skip_this_neighbor:
		inc tx
		ble tx 1 dx_loop
		inc ty
		ble ty 1 dy_loop

	ret


move_cursor:
	cmp key_code KEY_UP
	jmp.eq handle_cursor_move
	cmp key_code KEY_DOWN
	jmp.eq handle_cursor_move
	cmp key_code KEY_LEFT
	jmp.eq handle_cursor_move
	cmp key_code KEY_RIGHT
	jmp.eq handle_cursor_move

	ret

handle_cursor_move:
	col COLOR_GRID_LINE
	call update_cursor
	cmp key_code KEY_UP
	dec.eq cursor_y
	cmp key_code KEY_DOWN
	inc.eq cursor_y
	cmp key_code KEY_LEFT
	dec.eq cursor_x
	cmp key_code KEY_RIGHT
	inc.eq cursor_x

	# 回绕
	cmp cursor_x 0
	li.lt cursor_x 14
	cmp cursor_x GRID_COLS
	li.ge cursor_x 0
	cmp cursor_y 0
	li.lt cursor_y 7
	cmp cursor_y GRID_ROWS
	li.ge cursor_y 0
	col COLOR_CURSOR
	call update_cursor

	ret


update_cursor:
	clr i
	clr j

	mull x cursor_x TILE_SIZE
	mull y cursor_y TILE_SIZE

	update_cursor_loop1:
		spx x y
		inc x
		inc i
		ble i TILE_SIZE update_cursor_loop1
		mull x cursor_x TILE_SIZE
		clr i
		add y y TILE_SIZE
		inc j
		beq j 1 update_cursor_loop1

	mull x cursor_x TILE_SIZE
	mull y cursor_y TILE_SIZE

	update_cursor_loop2:
		spx x y
		inc y
		inc i
		ble i TILE_SIZE update_cursor_loop2
		mull y cursor_y TILE_SIZE
		clr i
		add x x TILE_SIZE
		inc j
		beq j 3 update_cursor_loop2

	ret


reveal_tile:
	mull addr cursor_y GRID_COLS
	add addr addr cursor_x
	lw t1 addr 0

	and t2 t1 REVEAL_MASK
	bne t2 0 reveal_tile_ret
	and t2 t1 FLAG_MASK
	bne t2 0 reveal_tile_ret
	and t2 t1 MINE_MASK
	bne t2 0 lose_loop
	and t2 t1 AROUND_COUNT_MASK

	mv arg_x cursor_x
	mv arg_y cursor_y

	cmp t2 0
	call.eq reveal_around
	or t1 t1 REVEAL_MASK
	sw addr t1 0

	cmp t2 1
	call.eq draw_num1

	cmp t2 2
	call.eq draw_num2

	cmp t2 3
	call.eq draw_num3

	cmp t2 4
	call.eq draw_num4

	cmp t2 5
	call.eq draw_num5

  reveal_tile_ret:
    ret


reveal_around:
	# 初始化当前坐标
	mv nx cursor_x
	mv ny cursor_y

	reveal_around_loop:
		# 越界检查
		blt nx 0 reveal_around_ret
		bge nx GRID_COLS reveal_around_ret
		blt ny 0 reveal_around_ret
		bge ny GRID_ROWS reveal_around_ret

		# 计算地址
		mull addr ny GRID_COLS
		add addr addr nx

		# 读取格子值
		lw t1 addr 0

		# 如果已揭示或有旗子，则返回
		and t2 t1 REVEAL_MASK
		bne t2 0 reveal_around_ret
		and t2 t1 FLAG_MASK
		bne t2 0 reveal_around_ret

		# 设置为已揭示
		or t1 t1 REVEAL_MASK
		sw addr t1 0

		# 获取周围雷数
		and t2 t1 AROUND_COUNT_MASK

		# 根据雷数绘图
		mv arg_x nx
		mv arg_y ny
		cmp t2 0
		col COLOR_REVEALED
		call.eq draw_tile
		cmp t2 1
		call.eq draw_num1
		cmp t2 2
		call.eq draw_num2
		cmp t2 3
		call.eq draw_num3
		cmp t2 4
		call.eq draw_num4
		cmp t2 5
		call.eq draw_num5

		# 如果不是空白格，不再递归
		bne t2 0 reveal_around_ret

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
	mull addr cursor_y GRID_COLS
	add addr addr cursor_x
	lw t1 addr 0

	and t2 t1 REVEAL_MASK
	cmp t2 0
	ret.ne

	and t2 t1 FLAG_MASK
	cmp t2 0

	dec.eq mine_num
	or.eq t1 t1 FLAG_MASK
	call.eq draw_flag

	inc.ne mine_num
  li.ne t2 0xFFFFFDF ; not.ne t2 FLAG_MASK
	and.ne t1 t1 t2
	mv arg_x cursor_x
	mv arg_y cursor_y
	col COLOR_HIDDEN
	call.ne draw_tile

	sw addr t1 0

	ret


draw_tile:
	clr i
	clr j
	mull x arg_x TILE_SIZE
	mull y arg_y TILE_SIZE
	inc x
	inc y
	draw_tile_loop:
		spx x y
		inc x
		inc i
		blt i 7 draw_tile_loop
		mull x arg_x TILE_SIZE
		inc x
		inc y
		clr i
		inc j
		blt j 7 draw_tile_loop

	ret


draw_flag:
	mull x cursor_x TILE_SIZE
	mull y cursor_y TILE_SIZE
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
	mull x arg_x TILE_SIZE
	mull y arg_y TILE_SIZE
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


draw_num1:
	col COLOR_REVEALED
	call draw_tile
	clr i
	clr j
	mull x arg_x TILE_SIZE
	mull y arg_y TILE_SIZE
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
	mull x arg_x TILE_SIZE
	mull y arg_y TILE_SIZE
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
	mull x arg_x TILE_SIZE
	mull y arg_y TILE_SIZE
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
	mull x arg_x TILE_SIZE
	mull y arg_y TILE_SIZE
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
	mull x arg_x TILE_SIZE
	mull y arg_y TILE_SIZE
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
  mv key_code kb
  beq key_code 0 read_key
  ret


main:
	call init_screen
	call init_mines
	call init_mine_counts

	main_loop:
    call read_key
		call move_cursor
		cmp key_code KEY_REVEAL
		call.eq reveal_tile
		cmp key_code KEY_FLAG
		call.eq toggle_flag
		seg mine_num
		jmp main_loop

	win_loop:
		jmp win_loop

	lose_loop:
		mv arg_x cursor_x
		mv arg_y cursor_y
		col COLOR_MINE_BACK
		call draw_tile
		call draw_mine

		clr ny
		clr addr
		lose_row_loop:
			clr nx
		lose_col_loop:
			bne nx cursor_x lose_loop_cont
			bne ny cursor_y lose_loop_cont
			jmp skip_draw
		lose_loop_cont:
			lw t1 addr 0
			and t2 t1 MINE_MASK
			cmp t2 0
			mv.ne arg_x nx
			mv.ne arg_y ny
			col COLOR_REVEALED
			call.ne draw_tile
			cmp t2 0
			call.ne draw_mine
		skip_draw:
			inc nx
			inc addr
			blt nx GRID_COLS lose_col_loop
			inc ny
			blt ny GRID_ROWS lose_row_loop

halt:
	jmp halt
