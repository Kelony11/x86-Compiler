# 🔧 x86-compiler
**Rust-Based Complete Compiler for Mini Expression Language & Control Flow**

# PROJECT OVERVIEW

This project implements a **two-stage Rust compiler toolchain** that lowers a small language into **x86-64 assembly** following the System V calling convention.

- `first/` – **Part 1**: expression-only language  
  - Input: `testN.exp` (e.g., `a * b + 5 * c`)  
  - Output: `foo` function in x86-64 assembly (`.s`)  
  - Used with small C drivers to run the expression.

- `second/` – **Part 2**: full mini-imperative language  
  - Supports: `args`, `int` declarations, assignment, `if/else`, `while`, `return`, comparisons.  
  - Input: `testN.rucomp`  
  - Output: `foo` function in x86-64 assembly (`.s`), run via C drivers.

All regression tests and C harnesses live under `tests/`, and a top-level `Makefile` automates building and running everything.

# ♺ x86-64 Code Generation 
    - Part 1: Expression → x86-64
        - Emits a function:
        ```
        .text
        .global foo
        foo:
            pushq %rbp
            movq %rsp, %rbp
            ...
            movq %rbp, %rsp
            popq %rbp
            ret
        ```
        - Arguments are assumed to be in:
        ```
        %rdi, %rsi, %rdx, %rcx, %r8, %r9
        ```
    
    - Part 2: Full Program → x86-64 with Stack Frame
        - Allocates stack slots for all args and local vars:
            - Each identifier → a negative offset from %rbp:
            ```
            pushq %rbp
            movq %rsp, %rbp
            subq $FRAME_SIZE, %rsp      # space for locals

            # store incoming arguments
            movq %rdi, -8(%rbp)         # a
            movq %rsi, -16(%rbp)        # b
            ...
            ```

# 🧱 TECHNICAL STACK 
- Languages / Tools
    - Rust + Cargo
    - C (for test harnesses)
    - Makefile (for regression automation)
    - clang (macOS, targeting -arch x86_64)


# 🚀 BUILD & RUN

The instructions to run & build this project is in another ReadME placed inside the `tests` folder 📁.


# 🤔 WHAT'S NEXT?

- Optimizations:
    - Common subexpression elimination for repeated subtrees (Part 1).
    - Dead code elimination in Part 2.

- Better register allocation:
    - Avoid push/pop for every operation.
    - Use more caller-saved registers intelligently.

- Extended language features:
    - Multi-argument functions, call/return.
    - Arrays or simple heap allocation.
    - More operators (division, modulo, logical &&, ||)

# 👤 Contributors 

- Kelvin Ihezue
