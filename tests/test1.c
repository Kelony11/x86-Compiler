#include <stdio.h>
#include <stdlib.h>


extern unsigned long foo(unsigned long a, unsigned long b, unsigned long c) __asm("foo");

int main() {
    
    printf("%lu\n", foo(7, 14, 6));
    return 0;
}
