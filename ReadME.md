# HOW TO EASILY RUN THE REGRESSION TESTS WITH Makefile


0. **Prerequisites**

- Rust + Cargo installed
- clang (or gcc) installed

(from the project root - rucompiler-x86):

1. **MakeFile**
    I included a MakeFile for easy testing of both first and second cargos. 

    from the root repo, command: make run(1/2)-all

        ``` Cd first/second && cargo build ``` 
        
        ...is done inside the makefile.


2. **PART ONE**

- **Run a single test**

    - example, run Part 1 test 1.exp: make run1-test1
        
    - You can do the same for any test (1-10)

- **Run all Part 1 tests at once**

    - make run1-all

- **Cleaning up build artifacts**

    - make clean


3. **PART TWO**

- **Run a single test**

    - example, run Part 2 test 1.rucomp: make run2-test1
        
    - You can do the same for any test (1-10)

- **Run all Part 1 tests at once**

    - make run2-all

- **Cleaning up build artifacts**

    - make clean



**Note for Grader**

1. **Part 2 harnesses are named testX_p2.c (Where X ranges inclusively from 1 to 10)**
    - Reason: I changed the test.c files for Second cargo  to avoid clashing with Part 1 testX.c harnesses. 

2. **About __asm("foo") in the c files. (macOS vs Linux)**

    ``` 
        extern unsigned long foo(...) __asm("foo");
    ```
    **If you are testing the files with a non-Mac computer**

    You can either:

    - Ignore it (most compilers just accept it), or

    - Use a simpler declaration in their own harness:

    ```
        extern unsigned long foo(unsigned long a, unsigned long b, unsigned long c);
    ```

    and link with:  (eg. test1)

    ```
        gcc test1.c test1.s -o test1.out
        ./test1.out
    ```






