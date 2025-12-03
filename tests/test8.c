#include <stdio.h>
#include <stdlib.h>

// 3 args: x, y, z  (in that order, from first-seen order in the expression)
extern unsigned long foo(unsigned long x,
                         unsigned long y,
                         unsigned long z) __asm("foo");

int main(void) {
    
    printf("%lu\n", foo(1, 2, 3));

    
    printf("%lu\n", foo(2, 3, 4));

    return 0;
}
