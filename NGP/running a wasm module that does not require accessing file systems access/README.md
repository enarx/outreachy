
## COMPILING C PROGRAM THAT DOES NOT ACCESS FILE SYSTEMS TO WASM AND RUNNING IT OUTSIDE WEB BROWSER USING THE COMMAND LINE INTERFACE
In this tutorial i will be compiling C programs that do not require access to file system to WASM and executing the WebAssembly module using wasmtime runtime. this execution is completely sand box since the program does not interact with external module.
the c program prints hello world ten times alongside a count value.

## Setting up enviroment
[Follow this tutorial to setup your development enviroment](https://github.com/paulnwoko/outreachy/tree/main/NGP/Setting%20up%20development%20enviroment%20for%20compiling%20and%20running%20wasm)


## Compiling to .WASM
open terminal and type in the following command. 

    ~/wasi-sdk-12.0/bin/clang {path to c source}hello.c --sysroot ~/wasi-sdk-12.0/share/wasi-sysroot/ -o {destination path for .wasm file}hello.wasm
    
here specify the directory path to the source and destination file

    ~/wasi-sdk-12.0/bin/clang ~/outreachy/enarx/demo/'new p'/hello.c --sysroot ~/wasi-sdk-12.0/share/wasi-sysroot/ -o ~/outreachy/enarx/demo/'new p'/hello.wasm
    
the directory now contains .wasm as shown below:
![image](https://user-images.githubusercontent.com/42975388/139480142-3a0d7561-99e4-4229-a0ec-cdab717a88c1.png)


#### Executing in wasmtime runtime
to run the webassembly module:

    wasmtime hello.wasm
    
result:  

    hello, world! : 0
    hello, world! : 1
    hello, world! : 2
    hello, world! : 3
    hello, world! : 4
    hello, world! : 5
    hello, world! : 6
    hello, world! : 7
    hello, world! : 8
    hello, world! : 9
    hello, world! : 10
    
the sceenshot is shown below:

![image](https://user-images.githubusercontent.com/42975388/139480355-e31586e5-b4cd-4f61-8585-5f6f1420d098.png)
