#include <stdio.h>
#include <stdint.h>

// __asm("foo") so the symbol name matches the assembly label `foo` on macOS
extern unsigned long foo(unsigned long a) __asm("foo");

int main(void) {

    printf("%lu\n", foo(0));  // expect 0
    printf("%lu\n", foo(3));  // expect 3
    printf("%lu\n", foo(4));  // expect 13
    printf("%lu\n", foo(6));  // expect 15

    return 0;
}
