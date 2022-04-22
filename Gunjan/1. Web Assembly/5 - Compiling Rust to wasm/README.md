# 4 - Compiling Rust to wasm

Hundreds of companies around the world are using Rust in production today for fast, low-resource, cross-platform solutions. Software you know and love, like Firefox, Dropbox, and Cloudflare, uses Rust. From startups to large corporations, from embedded devices to scalable web services, Rust is a great fit.

Rust is also, widely used in confidential computing applications to provide secure environments.

## Environment setup

- Rust

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
- Wasmtime

```
curl https://wasmtime.dev/install.sh -sSf | bash
```

- WebAssembly Rust toolchain

```
 rustup target install wasm32-wasi
```
There are 3 steps 
1. Compile the rust code
2. Compile rust code to WASM Binary
3. Execute it using WASM Runtime

So, lets start.

## Compile the Rust code

- make the new project at any preferred location in your machine

```
cargo new sum-of-2-nos
```
- open your favorite code editor and write rust code. (Here I 
  am demonstrating sum of 2 numbers) in main.rs.

```
use std::io;
use std::{i32};

fn main() {
          // User will enter first number

          println!("Input First number");

          let mut var1 = String::new();

          io::stdin().read_line(&mut var1).expect("Unable to read entered data");

          // User will enter Second number

          println!("Input second number");
          let mut var2 = String::new();
          io::stdin().read_line(&mut var2).expect("Unable to read entered data");

          // Converting string to integer
          let a: i32 = var1.trim().parse().ok().expect("Program only processes numbers, Enter number");
          let b: i32 = var2.trim().parse().ok().expect("Program only processes numbers, Enter number");

          // Output of basic operations
          println!("The sum of a &  b is : {}", a + b);

}
```
- complile and execute rust code.

```
cargo build
cargo run
```

![Terminal-ss](https://www.wasm.builders/remoteimages/uploads/articles/e4vebit2qiawaz6eypah.png)

## Compile Rust code to WASM Binary
It can be done using following command. 

```
cargo build --target=wasm32-wasi
```
We have now converted our rust code to web assembly code.


![ss](https://www.wasm.builders/remoteimages/uploads/articles/6yyw5wxnudfq19776fqn.png)

## Execute it using WASM Runtime

Execute the following command to see the results.

```
cd target/wasm32-wasi/debug
wasmtime sum-of-2-nos.wasm

```


![ss](https://www.wasm.builders/remoteimages/uploads/articles/52xa1subc6k78oryyntn.png)

Happy Hacking :)

[Tutorial for implementation](https://www.wasm.builders/gunjan_0307/compiling-rust-to-webassembly-5f32)
