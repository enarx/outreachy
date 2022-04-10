# Rust and Webassembly

## Installation

Cargo is the Rust package manager.Broadly speaking, a package manager is a program (or collection of related programs) in a software ecosystem that automates the process of obtaining, installing, and upgrading artifacts. Within a programming language ecosystem, a package manager is a developer-focused tool whose primary functionality is to download library artifacts and their dependencies from some central repository; this capability is often combined with the ability to perform software builds (by invoking the language-specific compiler).

 Cargo downloads your Rust package's dependencies, compiles your packages, makes distributable packages, and uploads them to crates.io, the Rust community’s package registry.

The easiest way to get Cargo is to install the current stable release of Rust by using rustup. Installing Rust using rustup will also install cargo.

On Linux and macOS systems, this is done as follows:

`curl https://sh.rustup.rs -sSf | sh`

It will download a script, and start the installation. If everything goes well, you’ll see this appear:

`Rust is installed now. Great!`

For other installation options and information, visit the [install](https://www.rust-lang.org/tools/install) page of the Rust website.

The cargo-wasi project is a subcommand for Cargo which provides a convenient set of defaults for building and running Rust code on the [wasm32-wasi target](https://wasi.dev/). 

Once you've got Rust installed you can install cargo-wasi with:

`$ cargo install cargo-wasi`

This will install a precompiled binary for most major platforms or install from source if we don't have a precompiled binary for your platform.

To verify that your installation works, you can execute:

`$ cargo wasi --version`

and that should print both the version number as well as git information about where the binary was built from.

Now that everything is set, let's build some code for wasi!

## Building from Source

Installing from crates.io via cargo install cargo-wasi will install precompiled binaries. These binaries are built on the cargo-wasi repository's CI and are uploaded to crates.io as part of the publication process. If you'd prefer to install from source, you can execute this command instead:

`$ cargo install cargo-wasi-src`



## Hello World!

Let's see an example of how to run the WASI version of "Hello, World!". 

```
$ cargo new wasi-hello-world
     Created binary (application) `wasi-hello-world` package
$ cd wasi-hello-world
```

This creates a `wasi-hello-world` folder which has a default `Cargo.toml` and `src/main.rs.` The `main.rs` is the entry point for our program and currently contains `println!("Hello, world!");`. Everything should be set up for us to execute (no code needed!) so let's run the code inside of the `wasi-hello-world` directory:

Ok, now that we've got a runtime installed, let's retry executing our binary:

```
$ cargo wasi run
info: downloading component 'rust-std' for 'wasm32-wasi'
info: installing component 'rust-std' for 'wasm32-wasi'
   Compiling wasi-hello-world v0.1.0 (/code/wasi-hello-world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
     Running `/.cargo/bin/cargo-wasi target/wasm32-wasi/debug/wasi-hello-world.wasm`
     Running `target/wasm32-wasi/debug/wasi-hello-world.wasm`
Hello, world!
```

Success! The command first used rustup to install the Rust wasm32-wasi target automatically, and then we executed cargo to build the WebAssembly binary. Finally wasmtime was used and we can see that Hello, world! was printed by our program. After this programs asks us to input two number. It returns the addition result.


## cargo wasi build

This is the primary subcommand used to build WebAssembly code. This will build your crate for the `wasm32-wasi` target and run any postprocessing (like `wasm-bindgen` or `wasm-opt`) over any produced binary.


`$ cargo wasi build --release`

Output `*.wasm` files will be located in `target/wasm32-wasi/debug` for debug builds or `target/wasm32-wasi/release` for release builds.