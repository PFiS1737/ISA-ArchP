# ArchP

一个存在于 [Turing Complete](https://store.steampowered.com/app/1444480/Turing_Complete/) 游戏中的简易指令集架构（ISA）。

这个仓库是其对应的汇编器。

## ISA

指令集的相关细节，见 [isa.txt](./isa.txt) 文件。

### Features

- 所有指令均可以条件执行 (predication)
- 允许最高 12bit 立即数直接参与运算
  - 更大的立即数，请使用 lui 指令先加载高位 20bit 立即数到寄存器，然后 ori 一个低 12bit 立即数
- b- 系列指令支持 12bit 绝对 PC 地址转跳
- call 指令支持 12bit 绝对 PC 地址转跳
- lw/sw 指令在内存中相对寻址，允许使用 12bit 正立即数偏移量
- rnd 指令基于系统时间生成伪随机数（仅沙盒模式）
- 像素显示屏相关指令和专用寄存器

### Known Issues

- 原则上不支持带符号数
- 无法直接相对 PC 进行相对转跳，~但提供 PC 地址伪寄存器~
- b- 系列指令不支持和立即数进行比较，请先 li 到寄存器
