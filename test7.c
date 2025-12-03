
#include <stdio.h>
#include <stdlib.h>

// 6 args, match first-seen order: a,b,c,d,e,f
extern unsigned long foo(
    unsigned long a,
    unsigned long b,
    unsigned long c,
    unsigned long d,
    unsigned long e,
    unsigned long f
) __asm("foo");

int main(void) {

    printf("%lu\n", foo(1, 1, 1, 1, 1, 1));
    return 0;
}
