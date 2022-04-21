# Executing your first C & Cpp program using WebAssembly

[# WebAssembly](https://webassembly.org/) (abbreviated Wasm) is a binary instruction format for a stack-based virtual machine. Wasm is designed as a portable compilation target for programming languages, enabling deployment on the web for client and server applications.


![WebAssembly](https://www.wasm.builders/remoteimages/uploads/articles/b729rjnehgef3nb4cisi.png)



## Setting Up C/Cpp and Wasm

Considering the enviornment in Linux

#### 1. Install C/Cpp compiler
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
wasmtime ReBinary.wasm 

```
If you want to compile a C/Cpp file to a WebAssembly WASI:

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
If you want to compile a C/Cpp file to plain WebAssembly:

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
$ curl https://wasmtime.dev/install.sh -sSf | bash
$ wasienv install-sdk unstable

```

### Run Your first Program
Create a file named `ReverseANo.c` or `ReverseANo.cpp` and write the Program

```
// Reverse a No.

#include<stdio.h>  
 int main()    
{    
int n, reverse=0, rem;    
printf("Enter a number: \n");    
  scanf("%d", &n);    
  while(n!=0)    
  {    
     rem=n%10;    
     reverse=reverse*10+rem;    
     n/=10;    
  }    
  printf("Reversed Number: %d \n",reverse);    
return 0;  
}  
```
Compile and run

Now, to generate `.wasm` binary file from the `.c/.cpp` file, we will use `Wasienv`.

For .c
```
wasicc ReverseANo.c -o ReBinary.wasm
```
For .cpp

```
wasicc ReverseANo.cpp -o ReBinary.wasm
```

Neglect the Warnings.

A new `ReBinary.wasm` file will be created.

Now, to execute this file we will use `wasmtime`.

```
wasmtime ReBinary.wasm 
```

You can run your first program by working on this example which is [Reverse a Number](https://github.com/enarx/outreachy/tree/main/aryankaushik/FunctionsInC_Wasm/ReverseANo) written in C.

Just change extension to .cpp for C++ programs
 
![Code preview](https://www.wasm.builders/remoteimages/uploads/articles/mhm22axlus92swmedng8.png)

![Finally](https://www.wasm.builders/remoteimages/uploads/articles/9frtb4uwme9kcw9lgvlw.jpg)



