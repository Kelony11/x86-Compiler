#include <stdio.h>
#include <stdint.h>

// __asm("foo") is for macOS so the symbol name matches the assembly label `foo`
extern unsigned long foo(unsigned long a) __asm("foo");

int main(void) {
    // sum_{i=0}^{a-1} i

    printf("%lu\n", foo(0));   // expect 0
    printf("%lu\n", foo(1));   // expect 0
    printf("%lu\n", foo(5));   // expect 10  (0+1+2+3+4)
    printf("%lu\n", foo(10));  // expect 45  (0..9)

    return 0;
}
