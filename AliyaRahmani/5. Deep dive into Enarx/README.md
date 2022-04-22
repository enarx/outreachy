![Image description](https://www.wasm.builders/remoteimages/uploads/articles/j7mpcmxxag5rt1l9dxp5.png)

# The ultimate guide to ENARX
Enarx allows developers to easily run applications in TEEs, without needing to be rewritten for particular platforms or SDKs. By encrypting and provisioning data and applications to secured “Keeps,” Enarx provides confidence that your data and applications are secure.

Enarx uses WebAssembly, or, to be more exact, WASI, which you can think of as a “headless” version of WebAssembly: whereas WebAssembly was originally designed to run within browsers, WASI-compliant runtimes support server-type applications.

Enarx is a framework for running applications in TEE instances–which we refer to as “Keeps” within the project–without the need to implement attestation separately, without the need to trust lots of dependencies and without the need to rewrite your application. It is designed to work across silicon architectures transparently to the user (you) so that your application can run on AMD silicon just as easily as it can run on Intel silicon, without having to recompile your code. As other TEE types become available, we plan to support them as well.

## BENEFITS
Enarx enables the simple deployment of workloads to TEEs on the public cloud. This provides a secure and isolated environment that prevents unauthorized access or modification of code. Based on WebAssembly, it allows developers to implement it using a wide variety of languages. And because it’s CPU-architecture independent, the same application code can be deployed across multiple targets. This also simplifies issues like cross-compilation and resolving different attestation mechanisms between hardware vendors.
### Installation of Enarx
Installing the required packages for running Enarx:

```
$ sudo apt update
$ sudo apt install git curl gcc pkg-config libssl-dev musl-tools python3-minimal
```
### Install Rust

```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
$ source $HOME/.cargo/env
```
### Install from GitHub
Installing Enarx using git clone:​

```
$ git clone https://github.com/enarx/enarx
$ cd enarx/
$ cargo build
$ cargo install --bin enarx --path ./
```
### Build and run a WebAssembly module​
Install the WebAssembly Rust toolchain:

```
$ rustup target install wasm32-wasi
```
We will be creating a simple guess game in rust.

```
$ cd ~/
$ cargo init --bin guess-game
$ cd guess-game
```

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/1dgckabtnjnqblffafmd.png)

Include the following code in main.rst:

```
use rand::Rng;
use std::io::BufRead;

fn main() {
    let mut rng = rand::thread_rng();
    let random = rng.gen_range(1..101);
    println!("Guess a number between 1 and 100");
    for line in std::io::stdin().lock().lines() {
        let parsed = line.ok().as_deref().map(str::parse::<i64>);
        if let Some(Ok(guess)) = parsed {
            match guess {
                _ if guess < random => println!("Too low"),
                _ if guess > random => println!("Too high"),
                _ => {
                    println!("That's right");
                    break;
                }
            }
        }
    }
}
```

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/we1azcynj1vohsamwunz.png)

Also, include rand dependency for random number generation in Cargo.toml file:

```
[dependencies]
rand = "*"
```

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/05wcwpw2lr264niwvbw3.png)

Compile the guess-game with the following command to wasm:

```
$ cargo build --release --target=wasm32-wasi
```

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/3m62a6m7sd9sg28q5sx4.png)

### Run
You can now execute the WebAssembly application in an Enarx keep if you installed the `enarx` binary and added it to your `$PATH`.

```
$ enarx run target/wasm32-wasi/release/guess-game.wasm
```

![Image description](https://www.wasm.builders/remoteimages/uploads/articles/hlyop9op3c8s2b6xd2gf.png)
Finally, we executed our first program using ENARX!!
