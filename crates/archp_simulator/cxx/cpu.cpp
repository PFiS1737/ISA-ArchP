#include "./cpu.hpp"

CPU::CPU(const char *file) {
  ctx = std::make_unique<VerilatedContext>();

  ctx->debug(0);
  ctx->threads(1);
  ctx->randReset(2);
  ctx->randSeed(std::time(0));

  top = std::make_unique<Vtop>(ctx.get());

  top->rst = 1;

#if VM_TRACE
  ctx->traceEverOn(true);
  tfp = std::make_unique<VerilatedFstC>();
  top->trace(tfp.get(), 99);
  tfp->open(file);
#endif
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

void CPU::eval() const {
  top->eval();
}

#if VM_TRACE
void CPU::dump() const {
  tfp->dump(ctx->time());
}
#endif

void CPU::finish() const {
  top->final();
#if VM_TRACE
  tfp->close();
#endif
}

std::unique_ptr<CPU> create_cpu(rust::String file) {
  return std::make_unique<CPU>(file.c_str());
}
