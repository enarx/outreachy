# Rust Compiled with Enarx

## Introduction
- With reference of [Enarx Docs](https://enarx.dev/docs/Start/Enarx), 
- The [Blog: Announcing Enarx](https://aliceevebob.com/2019/05/07/announcing-enarx/) by AUTHORS Mike Bursell and Yuki Kubota
- And Youtube Video: [Enarx for Confidential Computing - DevConf.CZ 2021](https://youtu.be/kheJFhljEck)

This explains magnificently how the framework Enarx runs applications in TEE instances and aims to minimize the trust relationships required when executing applications.

provides a WebAssembly runtime, based on wasmtime, offering developers a wide range of language choices for implementation, including Rust, C, C++, C#, Go, Java, Python, and Haskell.

Gives users a sense of extremely strong integrity and confidentiality protections of their sensitive data. 

In this blog, I'll be posting about setting up Enarx as a backend and the procedure of implementing **Enarx keeps** over the Rust project.

Before we start let me mention that the Enarx framework is hardware-specific i.e and OS as Linux is recommended if you are a Windows user.

It requires a CPU with a supported Trusted Execution Environment. and currently it supports **Intel SGX and AMD SEV-SNP"

- **Intel® Software Guard Extensions (Intel® SGX)**  helps protect data in use via unique application isolation technology. Protect selected code and data from modification using hardened enclaves with Intel SGX.

- **AMD's Secure Encrypted Virtualization (SEV)** allows the memory of virtual machines to be encrypted. This is a new feature for Linux's built-in Kernel-based Virtual Machine **(KVM)** hypervisor. 

The basic principle of SEV-SNP integrity is that if a VM is able to read a private (encrypted) page of memory, it must always read the value it last wrote.

I'm in Intel SGX support device so my focus will the same.

for Hardware requirements and for AMD SEV device support you can refer to [Enarx Docs](https://enarx.dev/docs/Installation/Requirements)

## Installing Enarx
### 1. Setting Up for SGX machine
- Run a recent kernel with SGX support compiled in
- Set the SGX device node permissions.
```
# groupadd -r sgx_prv
# cat > /etc/udev/rules.d/99-sgx.rules <<EOF

$ SUBSYSTEM=="misc", KERNEL=="sgx_provision", MODE="0660", GROUP="sgx_prv"
$ SUBSYSTEM=="misc", KERNEL=="sgx_enclave", MODE="0666"
```
Since everyone cannot access hardware with the support for Intel SGX or AMD SEV-SNP, so Enarx facilitates you with KVM support that can be run on devices other than this which have virtualization support.

> **KVM (Kernel-based Virtual Machine)** is a full virtualization solution for Linux on x86 hardware containing virtualization extensions (Intel VT or AMD-V).

KVM is loaded automatically by Linux Kernel.
you can check its presence by running the following command.
```
$ lsmod | grep kvm
```
expected output should be 
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/e1zy6mo5w8wrlmguu62b.png)

for `amd` `intel` would get replaced with it.

Now with this, you are done with Hardware requirements and certain permissions. Now it's time to set up your project, before that,

### 2. **[Install Dependencies](https://enarx.dev/docs/Installation/Setup)** 

#### Debian / Ubuntu
```
$ sudo apt update
$ sudo apt install git curl gcc pkg-config libssl-dev musl-tools python3-minimal
```
### 3. Install Rust

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# set path variable
$ source $HOME/.cargo/env
```
### 4. [Install Enarx](https://enarx.dev/docs/Installation/Enarx)
From **Github**
```
$ git clone https://github.com/enarx/enarx
$ cd enarx/
$ cargo build

$ cargo install --bin enarx --path ./
```
you can also install with [alternative methods](https://enarx.dev/docs/Installation/Enarx) like `crates.io` and `Nix`.

Since Enarx supports binary files for so to convert that we'll **Build and run a WebAssembly module**

### 5. Install WebAssembly Rust toolchain
```
$ rustup target install wasm32-wasi
$ cd~/
```
so now Enarx is installed in your system and it's globally accessible 
outside of the Enarx repository 

#### Setting Up a New Rust program.

### Hello World
```
$ cargo init --bin hello-world
$ cd hello-world
```
cargo new generates a “Hello, world!” program by default. in src/main.rs file.

Assuming you did install the enarx binary and have it in your $PATH, you can now run the WebAssembly program in an Enarx keep.

```
$ enarx run target/wasm32-wasi/release/hello-world.wasm
```

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/mavnjr916plsf1csx8hk.png)

### Guessing Game
```
$ cargo new guessing_game
$ cd guessing_game
```
Look at the generated Cargo.toml file: and update the dependencies section.
```
[package]
name = "guessing_game"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
rand="0.3.14"
```
#### Inside `src/main.lib` add:
```
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

```

Similar to the above compilation:
Compile the guessing_game into `.wasm`.
```
$ cargo build --release --target=wasm32-wasi
```
compile to Enarx Keep
```
$ enarx run target/wasm32-wasi/release/hello-world.wasm
```

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/8lai8k7zurvu00yk0bus.png)
 
To see what backends are supported on your system, run:
```
$ entry info
```
![Image description](https://www.wasm.builders/remoteimages/uploads/articles/ux8x7jjxy3rizpflyehe.png)
 
Here you can see that I my device `Backend: kvm` is supported and on manual selection `backend "sgx" is unsupported.` 
So am working with `backend=kvm target` 

You got the same out put then what happened in the backend you didn't got that right..? 
I'll be posting about that too soon on the basis of my understanding from the references mentioned above.






