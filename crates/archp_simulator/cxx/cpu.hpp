#pragma once

#include "verilated.h"
#include "verilated_fst_c.h"

#include "Vtop.h"

#if VM_TRACE
#include "rust/cxx.h"
#endif

class CPU {
public:
  CPU();
  ~CPU();

  bool got_finish() const;

  uint64_t time() const;
  void increase_time(uint64_t add) const;

  void flip_clk() const;
  void set_rst(bool rst) const;

  void eval() const;

#if VM_TRACE
  void init_trace(rust::String file) const;
  void dump() const;
#endif

  void finish() const;

private:
  mutable std::unique_ptr<VerilatedContext> ctx = nullptr;
  mutable std::unique_ptr<Vtop> top = nullptr;
  mutable std::unique_ptr<VerilatedFstC> tfp = nullptr;
};

std::unique_ptr<CPU> create_cpu();
