#pragma once

#include "verilated.h"
#include "verilated_fst_c.h"

#include "Vtop.h"

#include "rust/cxx.h"

class CPU {
public:
  CPU(const char *file);
  ~CPU();

  bool got_finish() const;

  uint64_t time() const;
  void increase_time(uint64_t add) const;

  void flip_clk() const;
  void set_rst(bool rst) const;

  void eval() const;

#if VM_TRACE
  void dump() const;
#endif

  void finish() const;

private:
  std::unique_ptr<VerilatedContext> ctx = nullptr;
  std::unique_ptr<Vtop> top = nullptr;
  std::unique_ptr<VerilatedFstC> tfp = nullptr;
};

std::unique_ptr<CPU> create_cpu(rust::String file);
