#include <stdio.h>
#include <stdlib.h>

// 3 arguments: a, b, c
extern unsigned long foo(unsigned long a, unsigned long b, unsigned long c) __asm("foo");

int main(void) {
    printf("%lu\n", foo(1, 2, 3));   // (1 + 2) * 3 = 9
    printf("%lu\n", foo(2, 5, 10));  // (2 + 5) * 10 = 70
    return 0;
}
