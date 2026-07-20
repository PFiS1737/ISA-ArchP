#include "./cpu.hpp"

CPU::CPU() {
  ctx = std::make_unique<VerilatedContext>();

  ctx->debug(0);
  ctx->threads(1);
  ctx->randReset(2);
  ctx->randSeed(std::time(0));

#ifdef VM_TRACE
  ctx->traceEverOn(true);
#endif

  top = std::make_unique<Vtop>(ctx.get());

  top->rst = 1;
}

CPU::~CPU() {
  top->final();
};

bool CPU::got_finish() const {
  return ctx->gotFinish();
}

uint64_t CPU::time() const {
  return ctx->time();
}

void CPU::increase_time(uint64_t add) const {
  ctx->timeInc(add);
}

void CPU::flip_clk() const {
  top->clk = !top->clk;
}

void CPU::set_rst(bool rst) const {
  top->rst = rst;
}

bool CPU::posedge_clk() const {
  return top->clk;
}

void CPU::eval() const {
  top->eval();
}

std::unique_ptr<CPU> create_cpu() {
  return std::make_unique<CPU>();
}
