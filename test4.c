
#include <stdio.h>
#include <stdlib.h>


extern unsigned long foo(unsigned long a,
                         unsigned long b,
                         unsigned long c,
                         unsigned long d) __asm("foo");

int main(void) {
   
    printf("%lu\n", foo(2, 3, 4, 5));
    return 0;
}
