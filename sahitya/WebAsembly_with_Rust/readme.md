# WebAssembly outside the browser

A simple demo of rust with WASM. 

# Introduction

<p align="justify"> WebAssembly also known as WASM (web assembly state machine) is not just a programming language or instruction set, it is also a compilation target for other languages to compile to, as well as a language in itself.


## Why WAAM outside the browser ?

- using webAssembly means using right tool for the job.
- WeAssembly proformance is very fase compare to modern programming languages like python, c++, javascript.
- Devloper can create really good web base software using webAssembly like video editing from the browser. 


# Environment setup

To compile this demo, you must install the some packages listed below.

### Rust


Go to [rust-lang.org](https://www.rust-lang.org/tools/install) and follow the instructions using rustup.

------------------------
### Wasmtime

You will find wasmtime at [wasmtime.dev](https://wasmtime.dev/)

We would create a simple program that caculates fibonacci sequence, According to the trusty Wikipedia, the Fibonacci sequence is characterized by the fact that every number after the first two is the sum of the two preceding ones.

create a folder with a name of your choice, i would be using "Webassembly-outside-the-browser" as the name of my folder.

-------------------------
## Install the WebAssembly Rust toolchain:
```bash
 rustup target install wasm32-wasi
```

go to directory
```bash
cd WebAssembly_with_Rust
```

## Rust Code
 
First create a new project using the command below:

```bash
cargo new demo
```

You can use any `IDE` of your choice and open up this project folder, add this to the main.rs file:

```rust
use std::io;

fn fib (n: u32) -> u32 {
    if n <= 0 {
        return 0;
    } else if n == 1 {
        return 1;
    }   fib(n - 1) + fib(n - 2)
 }

 fn main() {
    let mut nth = String::new();

    println!("Enter input: ");

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");

    let nth: u32 = nth.trim().parse().expect("Please type a number!");

    println!("Fibonacci: {}", fib(nth));
    
}
```

## Compiling Rust Code
1. Compile using cargo

```bash
cargo build
cargo run
```

2. Compile to wasm
```bash
cargo build --release --target=wasm32-wasi
``` 

3. The wasm file created in release folder of wasi32
```bash
file target/wasm32-wasi/release/demo.wasm
```

4. Wasm runtime
```bash
wasmtime target/wasm32-wasi/release/demo.wasm
```

output

```
Fibonacci generator
Type "quit" to end the program

Enter a positive integer:
8
21

Enter a positive integer:
10
55

Enter a positive integer:
quit
```