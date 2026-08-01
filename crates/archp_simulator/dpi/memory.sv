import "DPI-C" function int unsigned mem_load(
  input int unsigned addr,
  input bit [2:0] width
);

import "DPI-C" function void mem_store(
  input int unsigned addr,
  input bit [2:0] width,
  input int data
);

// TODO: remove this
import "DPI-C" function void pixel_display_set(
  input int unsigned x,
  input int unsigned y,
  input int unsigned color
);
