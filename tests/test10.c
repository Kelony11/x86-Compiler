#include <stdio.h>
#include <stdlib.h>

// 5 arguments: a, b, c, d, e in that order
extern unsigned long foo(unsigned long a,
                         unsigned long b,
                         unsigned long c,
                         unsigned long d,
                         unsigned long e) __asm("foo");

int main(void) {
    // Test 1: expect 47
    printf("%lu\n", foo(1, 2, 3, 4, 5));

    // Test 2: expect 14
    printf("%lu\n", foo(0, 1, 2, 3, 4));

    return 0;
}
