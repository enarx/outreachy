
## Setting up Development Enviroment
In this tutorial i will be setting up development enviroment for compiling C programs to WASM and executing the WASM module using wasmtime runtime.

## Download Clang, a C/C++ Compiler.
Upstream Clang and LLVM (from 9.0 onwards) can compile for WASI out of the box, and WebAssembly support is included in them by default. 

#### Download and install clang : 
    sudo apt install clang

when installation is complete, run this command to verify your installation.
    
    clang --version
    
result:
    
    clang version 10.0.0-4ubuntu1 
    Target: x86_64-pc-linux-gnu
    Thread model: posix
    InstalledDir: /usr/bin
    

## Install Runtime enviroment:
you could use either wasmtime or wasmer or other runtime enviroments

#### Install WASMTIME
The WASMTIME is a wasm runtime enviroment for running web assembly modules. The easiest way to install the wasmtime CLI tool is through our installation script. Linux and macOS users can execute the following:
    
    curl https://wasmtime.dev/install.sh -sSf | bash

This will download a precompiled version of wasmtime and place it in $HOME/.wasmtime, and update your shell configuration to place the right directory in PATH.
You can confirm your installation works by executing:

    wasmtime -V
    
result:    
    
    wasmtime 0.12.0

To run a wasm with wasmtime
    
    wasmtime <filename>.wasm

    
#### OR Install WASMER
WASMER is an open-source runtime for executing WebAssembly on the Server. To install it run the following command on yur terminal

    curl https://get.wasmer.io -sSfL | sh
    
To verify your installation:
    
    wasmer --version
result:
    
    wasmer 2.0.0

To run a wasm with wasmer

    wasmer run <filename>.wasm



## MY Reference:
about wasmtime: https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md#executing-in-wasmtime-runtime

about wasmer: https://wasmer.io/

