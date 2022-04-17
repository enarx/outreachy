According to the Stack Overflow Developer Survey 2021 conducted among more than 80,000 developers, Rust is the most loved programming language.

In 2020, Linux kernel developers proposed to write new Linux kernel code in Rust. To be clear, they didn't want to rewrite the entire Kernel, which was originally written in C, but to add new code in Rust that would work with the existing infrastructure. None other than Linus Thorvalds, the father of the open source operating system Linux, has embraced the idea and is eager to see the results of the project.

In addition, Google also plans to use Rust in the Linux kernel after bringing support for the Rust systems programming language to Android. The entire operation aims to reduce security gaps. Meanwhile, Microsoft turned to Rust to reduce memory-related bugs in Windows components.

About WASM, just to remind you Wasm is an intermediate binary format that serves as a compiler target for programming languages ​​like C, C++ and Rust.

To do your own version and try to practice go into this link to have the right setup. 
[https://enarx.dev/docs/WebAssembly/Rust]

This time as an example I will use a code that outputs the distance between Nairobi and El Paso.

To start use the commands: 
`cargo new demo`

Compile using cargo

`cargo build
cargo run`


![Image description](https://www.wasm.builders/remoteimages/uploads/articles/rgsoqcwgm82wx28xmvj5.png)


The code is the following:

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/vkovdsz9ouzbbeqiyyw8.png)

Compile to wasm

`cargo build --target=wasm32-wasi`

Wasm runtime


`wasmtime target/wasm32-wasi/release/demo.wasm`


![Image description](https://www.wasm.builders/remoteimages/uploads/articles/9bxyhu0zrcb1way49rlf.png)

1. https://codilime.com/blog/why-is-rust-programming-language-so-popular/

2. https://arstechnica.com/gadgets/2021/03/linus-torvalds-weighs-in-on-rust-language-in-the-linux-kernel/

3. https://thenewstack.io/rust-in-the-linux-kernel-good-enough/
