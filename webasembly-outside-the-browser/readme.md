# WebAssembly outside the browser

A simple demo of rust with WASM

# Introduction

<p align="justify"> WebAssembly also known as WASM (web assembly state machine) is not just a programming language or instruction set, it is also a compilation target for other languages to compile to, as well as a language in itself. 
WASM is augmentation of the abilities of JS by allowing other languages to operate in the browser, it executes binary code in the browser.

Wasmtime is a standalone wasm-only optimizing runtime for WebAssembly and WASI. It runs WebAssembly code outside of the Web and can be used both as a command-line utility or as a library embedded in a larger application. Install the wasmtime runtime to run the WebAssembly binary.

## Why WASM outside the browser?

Using WebAssembly means using the right tool for the job. For instance, who wants to write a banking app in JS?. If you're running anything that relies on mathematical numerical accuracy or speed that meant, until now, another AJAX call to have another language do alllllllll the math. With WebAssembly, we can do this in the browser, with, say, Rust.

Speed: for scripting languages like python and php, webassembly can hep you ge near native speed performance without he hassle of compiling a naive extension.

Lightweight Sandboxing: for low level languages like c/c++ and rust, webassembly can give you light weight sandboxing where the module cant access memory or resources unless they have been directly given to it, this makes reusing codes more secure.

For both scripting and low level languages, webassemble allows you to reuse code from any language without haing to rewrite it to a language of your choice.

## Environment setup

To compile this demo, you must install the following.

#### Rust

Go to [rust-lang.org](https://www.rust-lang.org/tools/install) and follow the instructions using rustup.

#### Wasmtime

You will find wasmtime at [wasmtime.dev](https://wasmtime.dev/)

We would create a simple program that caculates fibonacci sequence, According to the trusty Wikipedia, the Fibonacci sequence is characterized by the fact that every number after the first two is the sum of the two preceding ones.

create a folder with a name of your choice, i would be using "Webassembly-outside-the-browser" as the name of my folder.

```bash
cd Webassembly-outside-the-browser
cargo new --bin fibonacci
cd fibonacci
```

Cargo will create the necessary files including the directory the code will live in.

We can run the application using:

```bash
cargo run
``

Use you preferred IDE / editor, create `main.rs` file inside the `src` folder.
add the following:

```

use std::io;

fn fibonacci(n: u32) -> u32 {
match n {
0 => 0,
1 => 1,
\_ => fibonacci(n - 1) + fibonacci(n - 2),
}
}

fn main() {
println!("Fibonacci generator");
println!("Type \"quit\" to end the program");

    loop {
        let mut n = String::new();

        println!("\nEnter a positive integer:");

        io::stdin().read_line(&mut n).expect("Failed to read line");

        if n.trim() == "quit" {
            break;
        }

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("{}", fibonacci(n));
    }

}

````
Given an input integer, the program prints out its fibonacci, the program continues to run in a loop until user enters ```quit``` to stop the program.

We can run the application again using:

```bash
cargo run
````

We will create the WebAssembly module using the following command:

```
cargo +nightly build --target wasm32-wasi
output:
Compiling fibonacci v0.1.0 (/some/path/to/folder/fibonacci)
    Finished dev [unoptimized + debuginfo] target(s) in 0.34s
```

Now let us run the WebAssembly generated using wasmtime.

```bash
wasmtime target/wasm32-wasi/debug/fibonacci.wasm
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
