#include <stdio.h>
#include <stdint.h>

// Part 2: one argument `a`
// __asm("foo") so the symbol name matches the assembly label on macOS
extern unsigned long foo(unsigned long a) __asm("foo");

int main(void) {
    // Expect 0, 0, 0 for these positive inputs
    printf("%lu\n", foo(0));   // expect 0
    printf("%lu\n", foo(1));   // expect 0
    printf("%lu\n", foo(5));   // expect 0

    return 0;
}
