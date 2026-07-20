#include <fstream>
#include <iostream>
#include <stdexcept>
#include <string>

#include "Vtop__Dpi.h"

#include "./Program.hpp"

void Program::open(rust::Str file_name) const {
  std::ifstream ifs((std::string)file_name, std::ios::binary);
  if (!ifs) {
    throw std::runtime_error("Failed to open file: " + (std::string)file_name);
  }

  ifs.seekg(0, std::ios::end);
  size_t size = ifs.tellg();
  ifs.seekg(0, std::ios::beg);

  data.resize(size, 0);

  ifs.read(reinterpret_cast<char *>(data.data()), size);
}

uint32_t Program::get_instruction(uint32_t pc) {
  uint32_t instr = 0;

  for (int i = 0; i < 4; ++i) {
    uint32_t byte = 0;
    if (pc + i < data.size()) {
      byte = data[pc + i];
    }
    instr |= (byte << (8 * i));
  }

  return instr;
}

Program program;

unsigned int get_instruction(unsigned int pc) {
  return program.get_instruction(pc);
}
