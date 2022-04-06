[# WebAssembly](https://webassembly.org/) (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.


![Image description](https://www.wasm.builders/remoteimages/uploads/articles/b729rjnehgef3nb4cisi.png)



## Setting Up C and Wasm

Considering the enviornment in Linux

#### 1. Install C
`$ sudo apt-get update
$ sudo apt-get upgrade
$ sudo apt-get install build-essential
$ gcc -v
$ make -v`


#### 2. Install Wasmer
[Wasmer](https://docs.wasmer.io/integrations/c/setup) is an open-source runtime for executing WebAssembly on the Server.
If you haven't yet, let's install Wasmer:

```
$ curl https://get.wasmer.io -sSfL | sh
```

Once Wasmer is installed, you can get the pkg-config easily:

```
$ wasmer config --pkg-config
prefix=/Users/USER/.wasmer
exec_prefix=/Users/USER/.wasmer/bin
includedir=/Users/USER/.wasmer/include
libdir=/Users/syrus/.wasmer/lib

Name: wasmer
Description: The Wasmer library for running WebAssembly
Version: 2.0.0
Cflags: -I/Users/USER/.wasmer/include/wasmer
Libs: -L/Users/syrus/.wasmer/lib -lwasmer
```

#### 3. Install Wasienv
[Wasienv](https://github.com/wasienv/wasienv) is used to compile various programming languages to Wasm to run on browser or web sever.

```
curl https://raw.githubusercontent.com/wasienv/wasienv/master/install.sh | sh
```
If you want to compile a C file to a WebAssembly WASI:

```
# To compile to a WebAssembly WASI file
# This command will generate:
#  • An executable: ./example
#  • A WebAssembly file: ./example.wasm
wasicc examples/example.c -o example

# If you are using configure
wasiconfigure ./configure

# If you are using cmake (or make)
wasimake cmake .
```
If you want to compile a C file to plain WebAssembly:

```
# To compile to a WebAssembly file
# This command will generate:
#  • An executable: ./example
#  • A WebAssembly file: ./example.wasm
wasmcc examples/example.c -o example
```
For installing a SDK (wasienv install-sdk):
`wasienv install-sdk 7`

For setting a SDK as the default (wasienv default-sdk):
`wasienv default-sdk 7`


#### 4. Wasmtime
[Wasmtime](https://wasmtime.dev/) is a small and efficient runtime for WebAssembly & WASI.
```
curl https://wasmtime.dev/install.sh -sSf | bash

```

### Run Your first Program
You can run your first program by working on this example which is [Hello World](https://github.com/enarx/outreachy/tree/main/aryankaushik/FunctionsInC_Wasm/Hello) written in C.


Please Share your ideas and suggestions related to the Blog.
