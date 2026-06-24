#include "./top.hpp"

Top::Top(int argc, char **argv) {
  ctx = std::make_unique<VerilatedContext>();

  ctx->debug(0);
  ctx->threads(1);
  ctx->randReset(2);
  ctx->randSeed(std::time(0));
  ctx->commandArgs(argc, argv);

#ifdef VM_TRACE
  ctx->traceEverOn(true);
#endif

  top = std::make_unique<Vtop>(ctx.get());

  top->rst = 1;
}

Top::~Top() {
  top->final();
};

bool Top::gotFinish() const {
  return ctx->gotFinish();
}

uint64_t Top::time() const {
  return ctx->time();
}

void Top::increase_time(uint64_t add) {
  ctx->timeInc(add);
}

void Top::flip_clk() {
  top->clk = !top->clk;
}

void Top::set_rst(bool rst) {
  top->rst = rst;
}

bool Top::posedge_clk() const {
  return top->clk;
}

void Top::eval() {
  top->eval();
}

std::string Top::_commandArgsPlusMatch(const char *prefixp) {
  return ctx->commandArgsPlusMatch(prefixp);
}
