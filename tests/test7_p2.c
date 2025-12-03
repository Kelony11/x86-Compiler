#include <stdio.h>
#include <stdint.h>

// __asm("foo") ensures the symbol name matches the assembly label `foo` on macOS
extern unsigned long foo(unsigned long n) __asm("foo");

int main(void) {
    printf("%lu\n", foo(0));  // expect 0   (sum 0..-1 = 0)
    printf("%lu\n", foo(1));  // expect 0   (no inner iterations)
    printf("%lu\n", foo(4));  // expect 6   (0 + 1 + 2 + 3)
    printf("%lu\n", foo(5));  // expect 10  (0 + 1 + 2 + 3 + 4)
    return 0;
}
