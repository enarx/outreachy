WebAssembly a low-level assembly-like language with a compact binary format, is not a new programming language or a more suitably Compiler target.

We can't run high-level languages C/C++ directly into the browser rather we generally used GCC compiler for this in this tutorial, I'll be demonstrating how to compile high-level languages through WebAssembly,

Considering your operating system like Linux.
# Environment Setup
## 1. First, make sure your system is up to date
```
$ sudo apt-get update 
$ sudo apt-get upgrade
$ sudo apt-get install build essential
```
Verify installation
```
$ where is gcc
$ which gcc
$ gcc --version

# if you are compiling with make.
$ make --version

$ pip -v
```
If you are receiving a `pip: not found` message so run the below command and restart your terminal, don't worry These types of messages mostly occur at the time of installing `Wasienv`.
 > Tip: sometimes it becomes necessary to restart your terminal to get things to install properly.

```
$ sudo apt install python3-pip
```
Output shoud be somthing like: `pip 20.0.2 from /usr/lib/python3/dist-packages/pip (python 3.8)` may version can change for you.

For compiling C/C++ language it's recommended to have [Visual Studio code install](https://docs.microsoft.com/en-us/cpp/build/vscpp-step-0-installation?view=msvc-170) in your system this will make your coding experience bit smother.

## 2. Install [Wasmer](https://wasmer.io/)
Wasmer is an open-source runtime for executing WebAssembly on the Server. Wasmer allows you to run WebAssembly modules either Standalone or Embedded within other languages such as C/C++, Rust, Python, Go, PHP, Ruby...
```
$ curl https://get.wasmer.io -sSfL | sh
$ wasmer --version
```
Once Wasmer is installed, you can get the pkg-config.

```
$ wasmer config --pkg-config
```
you'll receive output something like this
```
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

## 3. Install Wasienv

```
$ curl https://raw.githubusercontent.com/wasienv/wasienv/master/install.sh | sh
```
If you want to compile a C/Cpp file to a WebAssembly WASI:
Replace example/examples with the name of your file.
```
# These are comments beginning with #
# To compile to a WebAssembly WASI file
# This command will generate:
#  • An executable: ./example
#  • A WebAssembly file: ./example.wasm
$ wasicc examples/example.c -o example

# If you are using configure
$ wasiconfigure ./configure

# If you are using CMake (or make)
$ wasimake cmake .
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

For setting an SDK as the default (wasienv default-sdk):
`wasienv default-sdk 7`
You can check its version by running the command
```
$ wasienv --version
```
for more commands in wasmenv you can refer to [Wasmev Git repo](https://github.com/wasienv/wasienv#commands)

# 4. Install [Wasmtime](https://wasmtime.dev/)
A small and efficient runtime for WebAssembly & WASI.
```
$ curl https://wasmtime.dev/install.sh -sSf | bash
```
You can check its version by running the command
```
$ wasmtime --version
```
# C++ 
With this, you are all set to run your first program. use either of them 
Open your Visual studio code with `$ code .` create a folder with .cpp extension in my case the folder name is HelloWasm.cpp
Try the basic **Hello Wasm** printing program 

```
#include <iostream>
#include <string>

int main()
{
    std::string firstname;
    std::string compiler;
    std::cout << "Hello User, Enter your first name.\n ";
    std::cin >> firstname;
    std::cout << "Enter your compiler name you gonna work with now.(gcc/wasienv)\n ";
    std::cin >> compiler;
    std::cout << "Hello " << firstname << ". you ran this program compiled through "<< compiler;
}
```
### or 

Try Printing **Pascal's triangle.**
repeat the same process.
Open your Visual studio code with `$ code .` create a folder with .cpp extension in my case the folder name is Pascal.cpp
```
#include <iostream>
using namespace std;

int main()
{
    int rows, coef = 1;

    cout << "Enter number of rows: ";
    cin >> rows;

    for(int i = 0; i < rows; i++)
    {
        for(int space = 1; space <= rows-i; space++)
            cout <<"  ";

        for(int j = 0; j <= i; j++)
        {
            if (j == 0 || i == 0)
                coef = 1;
            else
                coef = coef*(i-j+1)/j;

            cout << coef << "   ";
        }
        cout << endl;
    }

    return 0;
}
```

## To Compile with GCC compiler. 
This is the most common one to compile the C/C++ program. be careful with your file names since Linux is case-sensitive. (treats upper case and lowercase differently)
if you run  `$ g++ HelloWasm.cpp` without output (-o <outputfilename>) file name it creates `a.out` to represent the default output compiled file but just to not create confusion we'll provide a name to it.
```

$ g++ HelloWasm.cpp  -o HelloWasmOutput
# creates HelloWasmOutput.out

# now run the compiled file with
$ ./HelloWasmOutput
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/qybyp48ww8u79l7n74e8.png)
 
similarly for Pascal.cpp
```
g++ Pascal.cpp -o PascalOutput
./PascalOutput
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/zpydb0mpg2q15i5ae5uw.png)
 
## To compile with Wasienv compiler

Similar instruction as the above only difference would be your output will be with .wasm extension.
```
wasic++ HelloWasm.cpp -o HelloWasmOutput
wasmtime HelloWasmOutput
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/59dbnw2y0xh4kgkah3z6.png)
 
> Note that while executing this command, it might generate some warnings but you can ignore them.

Just like that for Pascal's Triangle.
```
wasic++ Pascal.cpp -o PascalOutput
wasmtime PascalOutput
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/7x2imu897dykxe62ra60.png)

# C
Pascal's Triangle in C
```
#include <stdio.h>
int main() {
   int rows, coef = 1, space, i, j;
   printf("Enter the number of rows: ");
   scanf("%d", &rows);
   for (i = 0; i < rows; i++) {
      for (space = 1; space <= rows - i; space++)
         printf("  ");
      for (j = 0; j <= i; j++) {
         if (j == 0 || i == 0)
            coef = 1;
         else
            coef = coef * (i - j + 1) / j;
         printf("%4d", coef);
      }
      printf("\n");
   }
   return 0;
}

```
## With GCC compiler
```
g++ Pascal.c -o Pascal_
./Pascal_C
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/82dg5diirhmgolbbwvhm.png)

## With Wasmenv compiler
``` 
wasicc Pascal.c -o Pascal_c
wasmtime Pascal_C.wasm
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/dy0h5e2kr3bz7dtnepxo.png)

Regardless of the Warning, you'll be getting your Tree printed.
Hope you enjoyed it compiling with wasmenv is as simple as with GCC only requirement is to set up your environment that too for once.
[Git repo for codes and their generated outputs](https://github.com/enarx/outreachy/tree/main/Kirtee) 