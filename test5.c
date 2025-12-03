#include <stdio.h>
#include <stdlib.h>


extern unsigned long foo(unsigned long a, unsigned long b) __asm("foo");

int main(void) {

    printf("%lu\n", foo(2, 4));
    return 0;
}
