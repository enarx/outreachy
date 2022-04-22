# Enarx: The future of Trusted Execution Environment Frameworks

![1](https://github.com/aryankaushik-git/outreachy/blob/main/aryankaushik/media/Enarx%20(1).png)

Hi Folks, Welcome to an another blog where we will learn about a leading Open Source Framework used for executing applications in TEEs (Trusted Execution Environments). It's part of the CCC(Confidential Computing Consortium) from the Linux Foundation.

![CCC](https://www.wasm.builders/remoteimages/uploads/articles/nle3moh16h6gwlqr0cjx.png)

**Introduction:**
Organizations from different sectors move their computing workloads across multiple environments, from on-premises to public cloud to Edge, they require greater assurances that their sensitive code and data are protected. This is especially true for sectors like banking & finance, government & public sector, telecommunications, Internet of Things, healthcare, customer data sensitive enterprise functions, defence, and human rights.

There are three states in which data can be protected: at rest, in transit, and in use. Encrypting data at rest and in transit have become a common practice in cloud computing, while encrypting data in use (the core idea behind Confidential Computing) is still an emerging concern:

a. Data at rest includes files, objects, and storage.

b. Data in transit includes data that is moving from one location to another such as across the Internet or a private network.

c. Data in use is data that is being processed in the CPU or memory.

![TEE](https://www.wasm.builders/remoteimages/uploads/articles/dtw7nivfsu16qk5235hn.png)
 

Trusted Execution Environments (TEEs) provide a hardware-based solution to this need to maintain data confidentiality and integrity in use, regardless of who might own or have access to the host system on which the application is running.

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/ajriye1dibusx6omeibk.png)
 

**Enarx** aims to minimize the trust relationships required when executing applications, meaning that the only components which need to be trusted are: the CPU and associated firmware, the workload itself, and the Enarx middleware, which is fully open source and auditable. Applications run without any of the layers in the stack (e.g. hypervisor, kernel, user-space) being able to look into or alter the Keep or its contents.

![Enarx](https://www.wasm.builders/remoteimages/uploads/articles/m04c84toekdxaiqawo15.png)
 
 
It is CPU-architecture independent, enabling the same application code to be deployed across multiple targets, abstracting issues such as cross-compilation and differing attestation mechanisms between hardware vendors.

Provides attestation, packaging and provisioning of the application to take place in a way which is transparent to the user. Every instance of an application goes through three steps:
- Attestation: Enarx checks that the host to which you’re planning to deploy is a genuine TEE instance.

- Packaging: Once the attestation is complete and the TEE instance verified, the Enarx management component encrypts the application, along with any required data.

- Provisioning: Enarx then sends the application and data along to the host for execution in the Enarx Keep.

Lets try it:
### 1. Install Rust

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# set path variable
$ source $HOME/.cargo/env
```
### 2. Install WebAssembly Rust toolchain

```
$ rustup target install wasm32-wasi
$ cd~/
```

### 3. Install Enarx from Github:

```
$ git clone https://github.com/enarx/enarx
$ cd enarx/
$ cargo build

$ cargo install --bin enarx --path ./
```
so now Enarx is installed in your system and it's globally accessible everywhere in the system.

Since Enarx supports binary files for so to convert that we'll Build and run a WebAssembly module.

### 4. Create a simple Rust program

```
cargo init --bin hello-world
$ cd hello-world
$ echo 'fn main() { println!("Hello, Enarx!"); }' > src/main.rs
$ cargo build --release --target=wasm32-wasi
```
You can now run the WebAssembly program in an Enarx keep.

```
$ enarx run target/wasm32-wasi/release/hello-world.wasm
[…]
Hello, Enarx!
```

Latest Version of Enarx is [Enarx 0.4.0](https://blog.enarx.dev/enarx-0-4-0-fort-dhat-al-hajj/): Fort of Dhat al-Hajj 
Check out the changelog for more details.

Enarx was built to be simple to use. It abstracts away complex concepts and supports multiple architectures transparently so that users don't have to worry about these.


Checkout Enarx on [Github](https://github.com/enarx/enarx) and give it a star.

Join the Community using this [Link](https://chat.enarx.dev/home), everyone is welcomed to share their contribution and thoughts.

References:
1. https://enarx.dev/docs
2. https://community.intel.com/t5/Blogs/Products-and-Solutions/Security/Confidential-Computing-the-emerging-paradigm-for-protecting-data/post/1335003
3. https://enarx.dev/resources/2022-04-12-enarx-0-4-4
4. https://www.gartner.com/smarterwithgartner/top-actions-from-gartner-hype-cycle-for-cloud-security-2020
5. https://www.ibm.com/cloud/learn/confidential-computing. 
