Just to have a disclaimer about the setup: 

The first step in converting a c file to wasm is to install the C language dependencies. Then it is necessary to make the following downloads in sequence:

**Wasmer:** is a WebAssembly runtime that enables super lightweight containers to run anywhere from Desktop to the Cloud, Edge and IoT devices.

**Wasienv:** ​Is a tool to make very easy to compile different programming languages to WebAssembly, so it is possible to run  programs  in the server.


**Wasmtime:** Is a Bytecode Alliance project that is a standalone wasm-only optimizing runtime for WebAssembly and WASI. It runs WebAssembly code outside of the Web, and can be used both as a command-line utility or as a library embedded in a larger application.

The links is at the end of the post. 

**Compiling the C Code:**

To compile the file is use the following command: 

`gcc NameOfTheFile.c`

Compile to WASM Binary using the following command:

Since the Code has been written in C, we need to figure out a way to generate a WebAssembly Binary. That's why we will be using wasienv in order to generate a .wasm binary from this c file.

When you have your c file created, you can execute wasicc:

` wasicc NameOfTheFile.c -o NameOfTheFile.wasm`

The exemple will be about find the minimum length unsorted subarray, sorting which makes the complete array sorted.

Given an unsorted array arr[0..n-1] of size n, find the minimum length subarray arr[s..e] such that sorting this subarray makes the whole array sorted. 

If the input array is [0, 1, 15, 25, 6, 7, 30, 40, 50], your program should be able to find that the subarray lies between the indexes 2 and 5.


![The first part of the code in VSC](https://www.wasm.builders/remoteimages/uploads/articles/m2efv65nn2fyltmss145.png)


![The second part of the code in VSC](https://www.wasm.builders/remoteimages/uploads/articles/cw5wrm7ifbsk24lqch62.png)

**Using the commands to compile:**

![The commands used to compile the file and turns it in a wasm file](https://www.wasm.builders/remoteimages/uploads/articles/tlhgmyndyfc6py2qn8s6.png)


![The wasmtime command being used.](https://www.wasm.builders/remoteimages/uploads/articles/8izbevom0ogzac23j60r.png)



1- [https://docs.wasmtime.dev/]
2- [https://enarx.dev/docs/WebAssembly/C]
3- [https://docs.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=msvc-170]
4- [https://docs.wasmer.io/]
5- [https://github.com/wasienv/wasienv]
6- [https://wasmtime.dev/]
