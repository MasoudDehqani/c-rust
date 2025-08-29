#include <stdio.h>
#include <limits.h>

int main()
{
  int max_int = 2147483647;
  int overflowed = max_int + 1;
  printf("%d\n", overflowed);
  return 0;
}

/*
  In a 3-bit unsigned binary number, for example, the maximum value is: 111 witch is 7 in decimal
  Representing 8 in would be 1000 in binary, but since the maximum in our 3-bit number is of
  3 bits, the extra one in the most significant place can't be displayed. So, 8 would be
  displayed as 0 in the 3-bit number, 9 would be 1, 10 would be 2 and so on...

  - It is equivalent to mod (modulo) function for unsigned int. For example in the 3-bit example, it is mod 8 (mod 2**3):
  examples:
  8 mod 8 -> 0
  9 mod 8 -> 1
  10 mod 8 -> 2
  formula for n bit unsigned data: mod 2**n

  - modulo VS modulus -> modulo is the operator, modulus is a noun which is the result of "x mod y"

  As in unsigned data the overflow will return back to its minimum, for signed data also the integer overflow will return
  back to its minumum. For example in a 4-bit signed int, adding 1 to the maximum which is 7, will set the value to -8.

  - Note that the representation of negative integers in C is based on the two's complement
*/
