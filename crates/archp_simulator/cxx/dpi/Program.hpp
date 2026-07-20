#pragma once

#include <cstdint>
#include <vector>

#include "rust/cxx.h"

class Program {
public:
  void open(rust::Str file_name) const;
  uint32_t get_instruction(uint32_t pc);

private:
  mutable std::vector<uint8_t> data;
};

extern "C" Program program;
