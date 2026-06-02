#include "archp_cpu.h"

uint64_t main_time = 0;

double sc_time_stamp() { return main_time; }

// 单周期推进（推荐封装）
void tick(archp_cpu *top) {
  // 上升沿
  top->clk = 1;
  top->eval();
  main_time++;

  // 下降沿
  top->clk = 0;
  top->eval();
  main_time++;
}

int main(int argc, char **argv) {
  Verilated::commandArgs(argc, argv);

  archp_cpu *top = new archp_cpu;

  // 初始化
  top->clk = 0;
  top->rst = 1;

  // ===== 复位阶段 =====
  for (int i = 0; i < 5; i++) {
    tick(top);
  }

  // 释放复位
  top->rst = 0;

  // ===== 正常运行 =====
  for (int i = 0; i < 300; i++) {
    tick(top);

    printf("cycle=%2d count=%d std_count=%d\n", i, top->count, top->std_count);
  }

  delete top;

  return 0;
}
