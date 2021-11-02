<h1> Let's do a "Hello world" program in Rust and executing in wasmtime runtime </h1>

## Pre-requisites :
Please ensure that you have installed [Rust and Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html)

## Tool Setup

We need to install WASM Pack

```bash
cargo install wasm-pack
```

You can see like this when you execute above command :

<img src="../../../images/Pasted image 20211031123450.png">


## Rust Code

 
On Cargo.toml we are going to add the next:

```rust
[package]
name = "demo"
version = "0.1.0"
authors = ["Shraddha Inamdar <shraddha.inamdar95@gmail.com>"]
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]


[[bin]]
path = "src/main.rs"
name = "main"

```

<h2>Writing Rust code</h2>

Writing a code for reading from file and writing to file using Rust.
  
  ```rust
  use std::env;
use std::fs;
use std::io::{Read, Write};

fn process(input_fname: &str, output_fname: &str) -> Result<(), String> {
    let mut input_file =
        fs::File::open(input_fname).map_err(|err| format!("Not Able To Open {}: {}", input_fname, err))?;
    let mut contents = Vec::new();
    input_file
        .read_to_end(&mut contents)
        .map_err(|err| format!("Not Able To Read: {}", err))?;

    let mut output_file = fs::File::create(output_fname)
        .map_err(|err| format!("Not Able To Open Output {}: {}", output_fname, err))?;
    output_file
        .write_all(&contents)
        .map_err(|err| format!("write error: {}", err))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    if args.len() < 3 {
        eprintln!("usage: {} <from> <to>", program);
        return;
    }

    if let Err(err) = process(&args[1], &args[2]) {
        eprintln!("{}", err)
    }
}
  
  ```
<p>&nbsp;</p>


## Compiling Rust Code
<p>&nbsp;</p>  

1. Compile using cargo

```bash
cargo build --target=wasm32-wasi
```

<img src="../../../images/Pasted image 20211031130744.png">


<p>&nbsp;</p>

2. The wasm file created in debug folder of wasi32

```bash
file target/wasm32-wasi/debug/main.wasm
```

<img src="../../../images/Pasted image 20211031130754.png">

<p>&nbsp;</p>

3. Wasm runtime

```bash

wasmtime target/wasm32-wasi/debug/main.wasm
```

<img src="../../../images/Pasted image 20211031130806.png">

<p>&nbsp;</p>

4. Created test.txt and output.txt with test.txt containing input

```bash
echo "Hi there, You made it! Cheers :)" > test.txt
```


```bash
touch output.txt
```

<img src="../../../images/Pasted image 20211031130817.png">

<img src="../../../images/Pasted image 20211031130825.png">

<p>&nbsp;</p>

5. Run using wasmtime

```bash

wasmtime target/wasm32-wasi/debug/main.wasm test.txt output.txt
```

<img src="../../../images/Pasted image 20211031130836.png">

<p>&nbsp;</p>

6. Solving the above error using –dir

```bash

wasmtime --dir=. --dir=. target/wasm32-wasi/debug/main.wasm test.txt output.txt
```

<img src="../../../images/Pasted image 20211031130847.png">


<p>&nbsp;</p>

7. Displaying the output

```bash
cat output.txt
```

<img src="../../../images/Pasted image 20211031130857.png">
	