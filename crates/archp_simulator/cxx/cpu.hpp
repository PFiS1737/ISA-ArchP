#pragma once

#include "Vtop.h"

class Top {
public:
  Top(int argc, char **argv);
  ~Top();

  bool gotFinish() const;

  uint64_t time() const;
  void increase_time(uint64_t add);

  void flip_clk();
  void set_rst(bool rst);
  bool posedge_clk() const;

  void eval();

  std::string _commandArgsPlusMatch(const char *prefixp);

private:
  std::unique_ptr<VerilatedContext> ctx = nullptr;
  std::unique_ptr<Vtop> top = nullptr;
};
