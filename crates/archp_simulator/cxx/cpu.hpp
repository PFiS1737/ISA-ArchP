#pragma once

#include "Vtop.h"

class CPU {
public:
  CPU();
  ~CPU();

  bool got_finish() const;

  uint64_t time() const;
  void increase_time(uint64_t add) const;

  void flip_clk() const;
  void set_rst(bool rst) const;
  bool posedge_clk() const;

  void eval() const;

private:
  std::unique_ptr<VerilatedContext> ctx = nullptr;
  std::unique_ptr<Vtop> top = nullptr;
};

std::unique_ptr<CPU> create_cpu();
