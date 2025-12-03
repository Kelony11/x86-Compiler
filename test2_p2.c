#include <stdio.h>
#include <stdint.h>

// One argument: a
// __asm("foo") ensures the C symbol is exactly "foo" (macOS quirk)
extern unsigned long foo(unsigned long a) __asm("foo");

int main(void) {
    // a = 5  (< 10)  → 5 + 1 = 6
    printf("%lu\n", foo(5));   // expect 6

    // a = 10 (!< 10) → 10 + 2 = 12
    printf("%lu\n", foo(10));  // expect 12

    return 0;
}
