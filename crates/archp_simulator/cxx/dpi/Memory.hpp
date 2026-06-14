#pragma once

#include <cstdint>
#include <vector>

class Memory {
public:
  void init(size_t size_in_bytes);
  void reset();

  uint32_t load(uint32_t addr, uint8_t width);
  void store(uint32_t addr, uint8_t width, uint32_t data);

private:
  std::vector<uint8_t> mem;
  size_t size = 0;
};
