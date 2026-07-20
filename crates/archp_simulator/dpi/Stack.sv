import "DPI-C" function void call_stack_reset();
import "DPI-C" function void call_stack_pop();
import "DPI-C" function void call_stack_push(input int data);
import "DPI-C" function int unsigned call_stack_peek();

import "DPI-C" function void data_stack_reset();
import "DPI-C" function void data_stack_pop();
import "DPI-C" function void data_stack_push(input int data);
import "DPI-C" function int unsigned data_stack_peek();
