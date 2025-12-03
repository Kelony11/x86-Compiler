#include <stdio.h>
#include <stdint.h>

// __asm("foo") so the symbol name matches the assembly label `foo` on macOS
extern unsigned long foo(unsigned long a) __asm("foo");

int main(void) {
    // a < 10  → expect 1
    printf("%lu\n", foo(5));

    // a >= 10 → expect 100
    printf("%lu\n", foo(10));
    printf("%lu\n", foo(25));

    return 0;
}
