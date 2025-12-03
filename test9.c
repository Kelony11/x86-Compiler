#include <stdio.h>
#include <stdlib.h>

// match arg order: x, y, z (first-seen order in the expression)
// __asm("foo") so mac uses symbol name "foo" exactly
extern unsigned long foo(unsigned long x,
                         unsigned long y,
                         unsigned long z) __asm("foo");

int main(void) {
    // x = 1, y = 3, z = 2
    // x*y + z*x + y = 1*3 + 2*1 + 3 = 3 + 2 + 3 = 8
    printf("%lu\n", foo(1, 3, 2));  // expect 8
    return 0;
}
