#pragma once

#include <cstdint>
#include <vector>

class Stack {
public:
  void reset();

  void pop();
  void push(uint32_t data);
  uint32_t peek() const;

private:
  std::vector<uint32_t> stack;
  size_t size = 0;
};
