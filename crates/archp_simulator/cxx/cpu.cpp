#include "archp_cpu.h"

// 单周期推进（推荐封装）
void tick(archp_cpu *top) {
  // 上升沿
  top->clk = 1;
  top->eval();

  // 下降沿
  top->clk = 0;
  top->eval();
}

int main(int argc, char **argv) {
  VerilatedContext *const contextp = new VerilatedContext;

  contextp->commandArgs(argc, argv);

  archp_cpu *top = new archp_cpu{contextp};

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

  // while (!contextp->gotFinish()) {
  //   top->eval();
  // }

  top->final();

  delete top;

  return 0;
}
