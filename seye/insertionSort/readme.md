### To compile this program

- Compile the C++ source code

    ```
    g++ insertion.cppp
    ```
- Ensure the program works by running

     ```
    ./a.out
    Output =  3 3 4 5 6 13 22 23 44 64 65 75 86 87 135 
    ```

- Compile to WASM Binary using the following Command:

    ```
    wasic++ insertion.cpp -o insertion.wasm
    ```

- Now, you will have a new FibonacciBinary.wasm file ready in your Directory Executing it using WASM Runtime

    ```
    wasmtime FibonacciBinary.wasm
    0utput ->  3 3 4 5 6 13 22 23 44 64 65 75 86 87 135 
    ```
Congratulations you just compiled your code to wasm.