# Fixed Partition Scheme


    The first attempt to allow for multiprogramming used fixed partitions (also called
    static partitions) within the main memory—one partition for each job. Because the
    size of each partition was designated when the system was powered on, each partition could only be reconfigured when the computer system was shut down, reconfigured, and restarted. Thus, once the system was in operation the partition sizes
    remained static.


## Algorithm to Load a Job in a Single-User System

    ```
        1 Store first memory location of program into base register (for memory protection)
        2 Set program counter (it keeps track of memory space used by the
        program) equal to address of first memory location
        3 Read first instruction of program
        4 Increment program counter by number of bytes in instruction
        5 Has the last instruction been reached?
            if yes, then stop loading program
            if no, then continue with step 6
        6 Is program counter greater than memory size?
            if yes, then stop loading program
            if no, then continue with step 7
        7 Load instruction in memory
        8 Read next instruction of program
        9 Go to step 4
    ```


### To compile this program

- To build this program

    ```
    cargo build
    ```
- To run this program

    ```
    cargo run
    ```

- Compile to WASM Binary using the following Command:

    ```
    cargo build --target=wasm32-wasi
    ```

- To run the wasm file run the command below
    ```
    wasmtime target/wasm32-wasi/debug/fixed_partition_scheme.wasm
    ```