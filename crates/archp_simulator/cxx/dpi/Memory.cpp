#include <algorithm>
#include <cstring>
#include <stdexcept>

#include "Vtop__Dpi.h"

#include "./Memory.hpp"

void Memory::init(size_t size_in_bytes) {
  mem.resize(size_in_bytes, 0);
  size = size_in_bytes;
}

void Memory::reset() { std::fill(mem.begin(), mem.end(), 0); }

uint32_t Memory::load(uint32_t addr, uint8_t width) {
  if (addr + width > size) {
    return 0;
  }

  switch (width) {
  case 1: {
    return mem[addr];
  }

  case 2: {
    uint16_t v;
    std::memcpy(&v, &mem[addr], 2);
    return v;
  }

  case 4: {
    uint32_t v;
    std::memcpy(&v, &mem[addr], 4);
    return v;
  }

  default:
    throw std::runtime_error("invalid width");
  }
}

void Memory::store(uint32_t addr, uint8_t width, uint32_t data) {
  if (addr + width > size) {
    return;
  }

  switch (width) {
  case 1:
    mem[addr] = static_cast<uint8_t>(data);
    break;

  case 2: {
    uint16_t v = static_cast<uint16_t>(data);
    std::memcpy(&mem[addr], &v, 2);
    break;
  }

  case 4: {
    uint32_t v = data;
    std::memcpy(&mem[addr], &v, 4);
    break;
  }

  default:
    throw std::runtime_error("invalid width");
  }
}

Memory mem;

void mem_reset() { mem.reset(); }

unsigned int mem_load(unsigned int addr, const svBitVecVal *width) {
  return mem.load(addr, *width);
}

void mem_store(unsigned int addr, const svBitVecVal *width, int data) {
  mem.store(addr, *width, static_cast<uint32_t>(data));
}
