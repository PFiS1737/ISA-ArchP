import "DPI-C" function int unsigned read_regfile(input bit [4:0] index);
import "DPI-C" function void write_regfile(
  input bit [4:0] index,
  input int data
);
