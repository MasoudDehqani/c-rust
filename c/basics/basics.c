#include <stdio.h>

int main() {
  printf("%d", sizeof(int));
  return 0;
}

/*

- Generally, in variable definition, we can say a value is bound to a name
- variables simply are names that point to a memory location

- declaration -> announcing properties of variable to the compiler
- definition -> allocating memory for a variable
- properties of variable -> name and size
- size is specified by type

- variable modifiers

- In C, int data type takes either 2 bytes or 4 bytes of memory, depending on the machine
- 1 byte is equal to 8 bits

- These are possible in C, but not in Rust:
int var1, var2, var3;
var1 = var2 = var3 = 10;
int x = 1, y = 2, z = 3;

- These are possible in Rust, but not in C:
let x = 1;
let x = 2;
(this is called shadowing in Rust)

- In C, starting variable names with _ is valid, but not recommended. Because huge number of variables starting with _ are reserved for
system use
- initialization -> assigning a value to variable

- placeholder for variable in a format string
- placeholders for different variables in format strings in C:
%d for decimal

- sizeof operator:
example -> sizeof(int)

*/
