CC         = clang
ARCH       = -arch x86_64
FIRST_BIN  = ./first/target/debug/rucompiler-x86-first
SECOND_BIN = ./second/target/debug/rucompiler-x86-second

# ------------ Part 1 tests (expression compiler) ------------
# Uses: testN.exp + testN.c  → testN.s + testN.out
P1_TESTS = test1 test2 test3 test4 test5 test6 test7 test8 test9 test10


# ------------ Part 2 tests (full program compiler) ----------
# Uses: testN.rucomp + testN_p2.c → testN_p2.s + testN_p2.out

P2_TESTS = test1 test2 test3 test4 test5 test6 test7 test8 test9 test10

.PHONY: all clean \
        build-first build-second \
        run1-% run1-all \
        run2-% run2-all

# ============================================================
# Part 1: Expression → x86 (first cargo)
# ============================================================

build-first:
	cd first && cargo build

# Build a Part 1 test: testN.out from tests/testN.exp + tests/testN.c
%.out: tests/%.exp tests/%.c
	$(FIRST_BIN) tests/$*.exp              
	$(CC) $(ARCH) -c tests/$*.c -o tests/$*_c.o
	$(CC) $(ARCH) -c tests/$*.s -o tests/$*_s.o
	$(CC) $(ARCH) tests/$*_c.o tests/$*_s.o -o $@


# Run a single Part 1 test: make run1-test1
run1-%: build-first %.out
	@echo "=== Part1 $* ==="
	./$*.out

# Run ALL Part 1 tests listed in P1_TESTS
run1-all: build-first $(P1_TESTS:=.out)
	@for t in $(P1_TESTS); do \
		echo "=== Part1 $$t ==="; \
		./$$t.out; \
	done


# ============================================================
# Part 2: Full program → x86 (second cargo)
# ============================================================

build-second:
	cd second && cargo build

# Build a Part 2 test: testN_p2.out from tests/testN.rucomp + tests/testN_p2.c
%_p2.out: tests/%.rucomp tests/%_p2.c
	$(SECOND_BIN) tests/$*.rucomp   
	mv tests/$*.s tests/$*_p2.s                
	$(CC) $(ARCH) -c tests/$*_p2.c -o tests/$*_p2_c.o
	$(CC) $(ARCH) -c tests/$*_p2.s -o tests/$*_p2_s.o
	$(CC) $(ARCH) tests/$*_p2_c.o tests/$*_p2_s.o -o $@


# Run a single Part 2 test: make run2-test10
run2-%: build-second %_p2.out
	@echo "=== Part2 $* ==="
	./$*_p2.out

# Run ALL Part 2 tests listed in P2_TESTS
run2-all: build-second $(P2_TESTS:%=%_p2.out)
	@for t in $(P2_TESTS); do \
		echo "=== Part2 $$t ==="; \
		./$${t}_p2.out; \
	done

# ============================================================
# Global helpers
# ============================================================

# Build everything (both compilers + all .out)
all: build-first build-second $(P1_TESTS:=.out) $(P2_TESTS:%=%_p2.out)

# Clean test artifacts (keeps Rust build artifacts under first/ and second/)
clean:
	rm -f *.o *.out *_p2.s tests/*.o tests/*.out tests/*_p2.s
