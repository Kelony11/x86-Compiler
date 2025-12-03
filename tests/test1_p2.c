#include <stdio.h>
#include <stdint.h>

// Part 2: one argument `a`
// __asm("foo") is for macOS so the symbol name matches the assembly label `foo`
extern unsigned long foo(unsigned long a) __asm("foo");

int main(void) {
    printf("%lu\n", foo(5));   // expect 6
    printf("%lu\n", foo(10));  // expect 11
    return 0;
}
