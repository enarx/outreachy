### To run this program as wasm

- Ensure the go program works by running

    ```
    go run main.go
    ```

- Compile to WASM Binary with tinygo:

    ```
    tinygo build -wasm-abi=generic -target=wasi -o main.wasm main.go
    ```

- Execute your complied wasm file using WASM Runtime

    ```
    wasmtime main.wasm
    ```
