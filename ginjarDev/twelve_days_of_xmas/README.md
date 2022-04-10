# Building a Simple WebAssembly App with Rust

JavaScript does have performance issues especially when performing resource intensive tasks like video streaming, interactive graphics, 3D rendering etc. JavaScript is meant to be a simple lightweight scripting language for adding interactivity to the web. However, that has changed as the posibilities of the web expanded. 

Due to the performance issues with JavaScript, WebAssembly was birth in 2017 to bring speed and better performance to the web. 

WebAssembly or simply ```wasm```, is a binary-code for executing programs on modern web browsers. It is a low-level assembly-like language that gives close performance speed like native languages such as **C**, **C++** and **Rust** with a compilation target so it can run on the web. It is small-sized, compiled and meant to run along with JavaScript. 

## Rust and WebAssembly 
Rust brings better performance to the web when compiled to WebAssembly because it has no unpredictable garbage collection pauses, JIT compiler performance cliffs. Pages load faster because of its small code size. 

Rust and WebAssembly can be used in two ways:
- For building an entire web application based in Rust.
- For building part of an application using Rust in an existing JavaScript frontend.

### Setting up Rust and WebAssembly on Windows Subsystem for Linux

Before you begin building your Rust and WebAssembly application, you will need to set up your environment on Windows Subsystem for Linux (WSL). Follow the commands below: 
```bash
$ sudo apt update
$ sudo apt install git curl gcc pkg-config libssl-dev musl-tools python3-minimal
```
#### Installing Rust
From your Windows Subssytem for Linux terminal run the following commands. 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
source $HOME/.cargo/env
rustup toolchain install nightly --allow-downgrade -t x86_64-unknown-linux-musl,x86_64-unknown-linux-gnu
```
- The first command ```curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y``` ensures a script is downloaded and installation follows immediately. 
- ```source $HOME/.cargo/env``` helps configure the Rust development environment variables where the Rust toolchain, ```rustc``` , ```cargo``` and ```rustup``` can be found. 
- The third command input creates a rustup toolchain for each of your rust-lang/rust workspaces and also installs nightly and downgrade rustup until you find the components needed.

#### Installing Wasmtime (WebAssembly runtime)
Run the following command in your terminal 
```bash
curl https://wasmtime.dev/install.sh -sSf | bash
```
This would create WebAssembly virtual environment as a portable compilation target for programming languages (e.g Rust), enabling deployment on the web for client and server applications. 

### Creating 12 Days of Christmas in Rust 
We would be writing a simple crate that would display the lyrics of the popular *12 Days Of Christmas* song. 

**Cargo** is the Rust package manager. It is a tool that allows Rust packages to declare their various dependencies and ensure that you’ll always get a repeatable build. This would reduce the need to specify any specific compiler flags or include external dependencies when executing a crate. 

Let's begin writing our program, follow the commands below.
```bash
cargo new twelve_days_of_xmas
cd twelve_days_of_xmas
```
- ```cargo new twelve_days_of_xmas``` creates a new package.
- Navigate into the new folder with ```cd twelve_days_of_xmas```.

By default Cargo generates this directory structure for us. To view this open your IDE preferably VS Code. 
```bash
$ cd twelve_days_of_xmas
$ tree .
.
├── Cargo.toml
└── src
    └── main.rs

1 directory, 2 files
```
In your ```src/main.rs```, you will find the venerable "Hello world!" program. 
```rust
fn main() {
    println!("Hello, world!");
}
```
Now we would replace the "Hello world!" program in ```main.rs``` with the new code for "Twelve Days of Christmas". 
```rust
fn day_intro(n: u32) {
    let day = match n {
        1 => "first",
        2 => "second",
        3 => "third",
        4 => "fourth",
        5 => "fifth",
        6 => "sixth",
        7 => "seventh",
        8 => "eighth",
        9 => "ninth",
        10 => "tenth",
        11 => "eleventh",
        12 => "twelfth",
        _ => "",
    };

    println!(
        "\nOn the {} day of Christmas\nmy true love sent to me:",
        day
    );
}

fn gift(n: u32, prefix: &str) {
    let gift_text = match n {
        1 => "a Partridge in a Pear Tree",
        2 => "Two Turtle Doves",
        3 => "Three French Hens",
        4 => "Four Calling Birds",
        5 => "Five Golden Rings",
        6 => "Six Geese a Laying",
        7 => "Seven Swans a Swimming",
        8 => "Eight Maids a Milking",
        9 => "Nine Ladies Dancing",
        10 => "Ten Lords a Leaping",
        11 => "Eleven Pipers Piping",
        12 => "12 Drummers Drumming",
        _ => "",
    };

    println!("{}{}", prefix, gift_text);
}

fn main() {
    println!("TWELVE DAYS OF CHRISTMAS:");

    for day in 1..13 {
        day_intro(day);

        for gift_day in (1..(day + 1)).rev() {
            gift(
                gift_day,
                if gift_day == 1 && day != 1 {
                    "and "
                } else {
                    ""
                },
            );
        }
    }
}
```
### Compiling Rust Program to WebAssembly
Switch back to your WSL terminal and run the following commands:
```rust
rustup target add wasm32-wasi
cargo build --target wasm32-wasi --release
```
- ```wasm32-wasi``` the Rust installer does not install the wasm32-wasi Rust standard library by default, but to compile any code for wasm32-wasi you'll need to be sure to have this target installed for your Rust toolchain.
- The seond command compiles your crate into WebAssembly module.

#### Running compiled WebAssembly 
Run the command below:
```
wasmtime ./target/wasm32-wasi/release/twelve_days_of_xmas.wasm
```
You should have your *12 Days of Christmas* lyrics displayed in your terminal like so:

> TWELVE DAYS OF CHRISTMAS:
> 
> On the first day of Christmas my true love sent to me: a Partridge in
> a Pear Tree
> 
> On the second day of Christmas my true love sent to me: Two Turtle
> Doves and a Partridge in a Pear Tree
> 
> On the third day of Christmas my true love sent to me: Three French
> Hens Two Turtle Doves and a Partridge in a Pear Tree
> 
> On the fourth day of Christmas my true love sent to me: Four Calling
> Birds Three French Hens Two Turtle Doves and a Partridge in a Pear
> Tree
> 
> On the fifth day of Christmas my true love sent to me: Five Golden
> Rings Four Calling Birds Three French Hens Two Turtle Doves and a
> Partridge in a Pear Tree
> 
> On the sixth day of Christmas my true love sent to me: Six Geese a
> Laying Five Golden Rings Four Calling Birds Three French Hens Two
> Turtle Doves and a Partridge in a Pear Tree
> 
> On the seventh day of Christmas my true love sent to me: Seven Swans a
> Swimming Six Geese a Laying Five Golden Rings Four Calling Birds Three
> French Hens Two Turtle Doves and a Partridge in a Pear Tree
> 
> On the eighth day of Christmas my true love sent to me: Eight Maids a
> Milking Seven Swans a Swimming Six Geese a Laying Five Golden Rings
> Four Calling Birds Three French Hens Two Turtle Doves and a Partridge
> in a Pear Tree
> 
> On the ninth day of Christmas my true love sent to me: Nine Ladies
> Dancing Eight Maids a Milking Seven Swans a Swimming Six Geese a
> Laying Five Golden Rings Four Calling Birds Three French Hens Two
> Turtle Doves and a Partridge in a Pear Tree
> 
> On the tenth day of Christmas my true love sent to me: Ten Lords a
> Leaping Nine Ladies Dancing Eight Maids a Milking Seven Swans a
> Swimming Six Geese a Laying Five Golden Rings Four Calling Birds Three
> French Hens Two Turtle Doves and a Partridge in a Pear Tree
> 
> On the eleventh day of Christmas my true love sent to me: Eleven
> Pipers Piping Ten Lords a Leaping Nine Ladies Dancing Eight Maids a
> Milking Seven Swans a Swimming Six Geese a Laying Five Golden Rings
> Four Calling Birds Three French Hens Two Turtle Doves and a Partridge
> in a Pear Tree
> 
>  On the twelfth day of Christmas my true love sent to me: 12 Drummers
> Drumming Eleven Pipers Piping Ten Lords a Leaping Nine Ladies Dancing
> Eight Maids a Milking Seven Swans a Swimming Six Geese a Laying Five
> Golden Rings Four Calling Birds Three French Hens Two Turtle Doves and
> a Partridge in a Pear Tree
