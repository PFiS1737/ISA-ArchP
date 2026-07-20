#include "Vtop__Dpi.h"

#include "./Stack.hpp"

void Stack::reset() {
  stack.clear();
}

void Stack::pop() {
  if (stack.empty()) {
    return;
  }

  stack.pop_back();
}

void Stack::push(uint32_t data) {
  stack.push_back(data);
}

uint32_t Stack::peek() const {
  if (stack.empty()) {
    return 0;
  }

  return stack.back();
}

Stack call_stack, data_stack;

void call_stack_reset() {
  call_stack.reset();
}
void call_stack_pop() {
  call_stack.pop();
}
void call_stack_push(int data) {
  call_stack.push(data);
}
unsigned int call_stack_peek() {
  return call_stack.peek();
}

void data_stack_reset() {
  data_stack.reset();
}
void data_stack_pop() {
  data_stack.pop();
}
void data_stack_push(int data) {
  data_stack.push(data);
}
unsigned int data_stack_peek() {
  return data_stack.peek();
}
