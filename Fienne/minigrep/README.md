# minigrep 
Command line tool built with Rust.
Goes through a provided file of text, and outputs the line(s) containing the piece of string being searched for.

## Requirements
[Rust](https://www.rust-lang.org/tools/install) should be installed on your system.

## Run
`cargo run searchstring example-filename.txt`

## Implement Using WebAssembly
* Install [Rust](https://www.rust-lang.org/tools/install)
* Install [wasmtime](https://wasmtime.dev)
* Install the WebAssembly Rust toolchain: `rustup target install wasm32-wasi`
* Compile Rust code: `cargo build` <br/>
`cargo run`
* Compile to wasm: `cargo build --target=wasm32-wasi`
* wasm runtime: `wasmtime target/wasm32-wasi/release/demo.wasm`

