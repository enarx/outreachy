# 3 - Enarx Demo : Confidential Computing - Part 3

## Enarx

- It is the leading open source framework for running applications in TEEs (Trusted Execution Environments). It's part of the Confidential Computing Consortium from the Linux Foundation.

- It provides a run-time TEE based on WebAssembly, allowing developers to deploy applications without any rewrites from languages like Rust, C/C++, C#, Go, Java, Python, Haskell and many more.

## Setting up enarx environment

There are basically 3 major steps :
Before we start please check out the [system requirements](https://enarx.dev/docs/Installation/Setup) 
_Linux distribution - Ubuntu 21.10_

1) Initial setup
2) Installing Enarx
3) Running Enarx

**Initial Setup**

```
$ sudo apt update
$ sudo apt install git curl gcc pkg-config libssl-dev musl-tools python3-minimal
```
Also, we will need rust and WebAssembly Rust toolchain:

Rust 
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
$ source $HOME/.cargo/env
```
WebAssembly Rust Toolchain

```
$ rustup target install wasm32-wasi
```
**Installing Enarx**

Enarx can be installed from github as follows:

```
$ git clone https://github.com/enarx/enarx
$ cd enarx/
$ cargo build
$ cargo install --bin enarx --path ./
```
**Running Enarx**
Let's create a simple hello world rust program by :

```
$ cd ~/
$ cargo init --bin hello-world
$ cd hello-world
$ echo 'fn main() { println!("Hello, Enarx!"); }' > src/main.rs
$ cargo build --release --target=wasm32-wasi
```
You can now run the WebAssembly program in an Enarx keep.

```
enarx run target/wasm32-wasi/release/hello-world.wasm
```
You should see your output :

![output](https://www.wasm.builders/remoteimages/uploads/articles/jgoic17g3s024uy39hrq.png)

**Running on different backend**
To see what backends are supported on your system, run:

```
$ enarx info
```
As, you can see in the below screenshot, my computer supports kvm, so I use kvm to run my backend
![backend info](https://www.wasm.builders/remoteimages/uploads/articles/zpg85zjtb4yumup6f9r0.png)

```
enarx run --backend=kvm target/wasm32-wasi/release/hello-world.wasm
```

![output](https://www.wasm.builders/remoteimages/uploads/articles/pl9wbaeaqlxjvte2yv2v.png)

In this post, I have shared my journey for setting up enarx, but there are multiple ways to do so, if you want to do it in another way, please refer [enarx](https://enarx.dev/docs/Installation/Wasm)

Happy Coding!!

[Blog Post](https://www.wasm.builders/gunjan_0307/enarx-demo-confidential-computing-part-3-1ni8)
