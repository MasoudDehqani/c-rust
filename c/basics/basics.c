#include <stdio.h>
#include <limits.h>

int main()
{
  signed int minimum_of_int = INT_MIN;
  signed int maximum_of_int = INT_MAX;

  unsigned int minimum_of_unsigned_int = 0;
  unsigned int maximum_of_unsigned_int = UINT_MAX;

  short int minimum_of_short_int = SHRT_MIN;
  short int maximum_of_short_int = SHRT_MAX;

  unsigned short int minimum_of_unsigned_short_int = 0;
  unsigned short int maximum_of_unsigned_short_int = USHRT_MAX;

  long int minimum_of_long_int = LONG_MIN;
  long int maximum_of_long_int = LONG_MAX;

  unsigned long int minimum_of_unsigned_long_int = 0;
  unsigned long int maximum_of_unsigned_long_int = ULONG_MAX;

  printf("the range of int is: %d to %d\n", minimum_of_int, maximum_of_int);
  printf("the range of unsigned int is: %u to %u\n", minimum_of_unsigned_int, maximum_of_unsigned_int);
  printf("the range of short int is: %d to %d\n", minimum_of_short_int, maximum_of_short_int);
  printf("the range of unsigned short int is: %u to %u\n", minimum_of_unsigned_short_int, maximum_of_unsigned_short_int);
  printf("the range of long int is: %ld to %ld\n", minimum_of_long_int, maximum_of_long_int);
  printf("the range of unsigned long int is: %lu to %lu\n", minimum_of_unsigned_long_int, maximum_of_unsigned_long_int);
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
- sizeof is an operator, not a function

- range -> upper and lower limit of some set of data

*/
