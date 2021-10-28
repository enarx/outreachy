
## COMPILING C/C++ TO WASM AND RUNNING IT OUTSIDE WEB BROWSER USING THE COMMAND LINE INTERFACE
In this tutorial i will be compiling C programs to WASI and executing the compiled WebAssembly module using wasmtime runtime.

## Setting up enviroment
Upstream Clang and LLVM (from 9.0 onwards) can compile for WASI out of the box, and WebAssembly support is included in them by default. 

#### Download and install clang : 
    sudo apt install clang


## Compiling to .WASM

The wasi-sdk provides a clang which is configured to target WASI

use the comand to generate a webassembly file in pure binary format:

    ~/wasi-sdk-12.0/bin/clang {path to c source}demo.c --sysroot ~/wasi-sdk-12.0/share/wasi-sysroot/ -o {destination path for .wasm file}demo.wasm
    
here specify the directory path to the source and destination file

    ~/wasi-sdk-12.0/bin/clang /home/ngp/outreachy/enarx/demo/'c-cpp to wasm outside browser'/demo.c --sysroot ~/wasi-sdk-12.0/share/wasi-sysroot/ -o /home/ngp/outreachy/enarx/demo/'c-cpp to wasm outside browser'/demo.wasm

    
After running the above comand a .wasm file was generated in the /home/ngp/outreachy/enarx/demo/'c-cpp to wasm outside browser'/ directory as show below
![image](https://user-images.githubusercontent.com/42975388/139062884-eadd5875-7eed-4bea-ad19-531ca2017786.png)


## To run the wasm file

First Install wasmtime if not already installed in your machine. 
The wasmtime is a wasm runtime enviroment
The easiest way to install the wasmtime CLI tool is through our installation script. Linux and macOS users can execute the following:
    
    curl https://wasmtime.dev/install.sh -sSf | bash

This will download a precompiled version of wasmtime and place it in $HOME/.wasmtime, and update your shell configuration to place the right directory in PATH.
You can confirm your installation works by executing:

    wasmtime -V
    
    wasmtime 0.12.0


#### Executing in wasmtime runtime

the program requires a command line argument so let's parse it as shown below:

create a file test and write hello world to it. the c application will used this file
    echo hello world > test.txt
    
    wasmtime demo.wasm test.txt /home/ngp/outreachy/enarx/demo/'c-cpp to wasm outside browser'/somewhere.txt
    
result:

    error opening input test.txt: Capabilities insufficient

The program is attempting to access a file by the name of test.txt, but was denied access. The is the sandboxing in action. 
the outcome is show in the screenshot below.
![image](https://user-images.githubusercontent.com/42975388/139066691-8fa73540-22b4-4f98-b6c5-ae2fbd9f52ae.png)


to give access to the file system, specify the directory path to the .wasm file to run, .txt file to read from and the .txt file to create and write to accordingly.

    wasmtime --dir=. --dir=/home/ngp/outreachy/enarx/demo/'c-cpp to wasm outside browser'/ demo.wasm test.txt /home/ngp/outreachy/enarx/demo/'c-cpp to wasm outside browser'/somewhere.txt

to read to content of the the somewhere.txt

    cat /home/ngp/outreachy/enarx/demo/'c-cpp to wasm outside browser'/somewhere.txt
    
result:  

    hello world
    
the sceenshot is shown below:

![image](https://user-images.githubusercontent.com/42975388/139064974-468ec883-9648-44df-93ef-c468cc659cf8.png)


the directory now contains all the generated files (.wasm, and .txt files) as shown below

![image](https://user-images.githubusercontent.com/42975388/139068579-35a4db27-e692-4caf-9fa5-69da8e417ced.png)


## MY Reference:
I followed this tutorial https://github.com/bytecodealliance/wasmtime/blob/main/docs/WASI-tutorial.md#executing-in-wasmtime-runtime
The c source i'm working with is also from there.


