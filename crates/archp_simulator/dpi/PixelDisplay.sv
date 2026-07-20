import "DPI-C" function void pixel_display_reset();

import "DPI-C" function void pixel_display_set(
  input int unsigned x,
  input int unsigned y,
  input int unsigned color
);

// TODO: move to other file
import "DPI-C" function int unsigned keyboard_get();
