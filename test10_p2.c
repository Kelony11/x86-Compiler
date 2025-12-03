#include <stdio.h>
#include <stdint.h>

// Part 2: one argument `a`
// __asm("foo") is for macOS so the symbol name matches the assembly label `foo`
extern unsigned long foo(unsigned long a) __asm("foo");

int main(void) {

    // a = 2  -> expect 31
    printf("%lu\n", foo(2));

    // a = 5  -> expect 34
    printf("%lu\n", foo(5));

    // a = 7  -> expect 32
    printf("%lu\n", foo(7));

    return 0;
}
