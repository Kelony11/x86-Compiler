#include <stdio.h>
#include <stdlib.h>


extern unsigned long foo(unsigned long a, unsigned long b) __asm("foo");

int main(void) {
 
    printf("%lu\n", foo(4, 1));   // expect 24
    return 0;
}
