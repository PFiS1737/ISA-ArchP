import "DPI-C" function void mem_reset();

import "DPI-C" function int unsigned mem_load(
  input int unsigned addr,
  input bit [2:0] width
);

import "DPI-C" function void mem_store(
  input int unsigned addr,
  input bit [2:0] width,
  input int data
);
