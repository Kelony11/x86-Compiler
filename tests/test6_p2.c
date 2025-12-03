#include <stdio.h>
#include <stdint.h>

// Part 2: one argument `a`
// __asm("foo") ensures the symbol name matches the assembly label `foo` on macOS.
extern unsigned long foo(unsigned long a) __asm("foo");

int main(void) {
   
    printf("%lu\n", foo(1));   // expect 8

    printf("%lu\n", foo(4));   // expect 14

    return 0;
}
