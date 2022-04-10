# WASM  Setup Guide for C++

This tutorial assumes the user is running Ubuntu or a Debian based distibution as their primary OS, despite this commands for non-debian distros are provided.

## Step 1 - Install the GCC compiler for C++

The best way to install the gcc compiler is to install the **build essential** package with the command   

    ```
    sudo apt install build essential

    <!-- Red Hat Distros -->
    sudo yum groupinstall "Development Tools" 
    
    ```

alternatively we can also install the compiler directly with

    ```
    sudo apt install g++
    ```

## Step 2 - Install Additional packages.

We are going to be installing wasmer, wasienv so we might need some extra dependencies to be installed before running installing them.

- Debian/Ubuntu

```
    sudo apt update
    sudo apt install git curl gcc pkg-config libssl-dev musl-tools python3-minimal
```

- Fedora     

    ```
    sudo dnf install git curl gcc pkg-config openssl-devel musl-gcc
    ```

- CentOS 8

    ```
    sudo dnf copr enable ngompa/musl-libc
    sudo dnf install git curl gcc-toolset-9 openssl-devel musl-gcc
    source "/opt/rh/gcc-toolset-9/enable"    
    ```

- Install Rust

    ```
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
    ```


## Step 3 - Install wasmer and wasienv
Wasmer is an open-source runtime for executing WebAssembly on the Server. You can install the Wasmer Standalone runtime with the command 

    ```
    curl https://get.wasmer.io -sSfL | sh

    <!-- You can check if wasmer is properly installed by checking your wasmer version.-->
    wasmer --version
    ```

Wasienv is a tool that aims to bring all programming languages to WebAssembly WASI. With wasienv you can compile C,C++/Swift to wasm + WASI.You can install wasienv with:

    ```
    curl https://raw.githubusercontent.com/wasienv/wasienv/master/install.sh | sh
    
    ```
Wasmtime is a small and efficient runtime for WebAssembly & WASI, We will nedd to run our compiled wasm file from our C++ program.

    ```
    curl https://wasmtime.dev/install.sh -sSf | bash
    
    ```
So you might be asking why do we need all this ? The end goal is to run our wasm application on enarx. Enarx provides a Webassembly runtime for your applications to run, so there is need to compile your programs to a wasm format for it to run on enarx. 
Now that we are done setting up, we can move to some tutorials to see how c++ programs and wasm works with enarx.

## Step 4 - Run your first program on wasm
- [You can run your first program by working on this example which is an insertion sort technique written in C++](./insertionSort)
