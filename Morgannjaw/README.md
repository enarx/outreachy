 # What is Web Assembly and how does it work?

Most WebAssembly tutorials and examples you will find online focus on using WebAssembly inside the browser in order to accelerate various functionality of a website or web app.
However, in this tutorial we’re going to focus on another none less important area of application “ outside the browser”.

### What is Web Assembly?

Web Assembly, also known as Wasm is an open standard, or code format that allows the execution of binary code on a web browser. Web Assembly is more of an improvement for JS and not a replacement.  It is designed to make large applications run on a browser with near native performance and will run native code such as C/C++ and Rust (things which will interact with the computer but on a browser and helps the browser understand binary format). 
Web assembly defines an Abstract Syntax Tree (AST) (i.e., a way to parse C/C++ into binary) that gets stored in a binary format which can then create smaller app bundles. The AST will be represented in a friendly text format. 


## Getting Started with Web Assembly.

###	Basic Concepts.

Before we get into more detail about Web Assembly, we need to familiarize ourselves with some basic concepts to understand how it runs in and out of the browser. We shall discuss about Modules and Stack Machines.

#### Modules.
Web Assembly is organized into modules which are compiled in binary by the browser (or virtual machine). Each module can contain functions and can import and export other functions. The code below illustrates a simple module which takes two 32-bit parameters, add them up and return the function.
```
(module
    (func $add (param $lhs i32) (param $rhs i32) (result i32)
        (i32.add
            (get_local $lhs)
            (get_local $rhs)
        )
    )
    (export "add" (func $add))
)
```
 
#### Stack Machine.

Web Assembly is a stack machine because it works under a system of instructions (ISA- Instruction Set Architecture). These instructions allow the control of the loops, arithmetic operations, and memory access. 
In a stack machine, most of the instructions assume that the operands are sitting on the stack, rather than stored in specified registers. The WebAssembly stack is a LIFO (Last In, First Out) stack. If you’re unfamiliar with the concept of a stack: it is as its name implies— values are piled (stacked) on top of each other, and unlike arrays where you can access any data regardless of location in the pile, stacks only allow you to pop data off or push data onto the top.
To add two numbers in a stack machine, you push those numbers onto the top of the stack. Then you push the ADD instruction onto the stack. The two operands and the instruction are then popped off the top and the result of the addition is pushed on in their place

For example, in the code above.

-	```get_local $lhs```  means gets the first parameter that was passed into this function and 
-	```get_local $rhs``` gets the value of b and place on top of a in the stack. 
-	The function ```i32.add```, receives two 32-bit integers and returns the result of their addition.


## Why would we want to run WebAssembly outside of a browser?

The main advantages of running WebAssembly outside the browser is that it provides system level access without compromising on security of its host. This is done through WASI (the Web Assembly System Interface). WASI is a collection of C-like functions that provide access to functionality such as ```fd_read, rand, fd_write```, threads (WIP), in a safe way.

### Basic Example – Web assembly Using Rust.  

For this tutorial we are going to use MacOS, Visual Studio Code ( or any text editor of your choice) and of course,  Rust. So we will need to install Rust from its [official website](https://www.rust-lang.org/tools/install). I suggest using  VsCode as your text editor and you’ll install some tools on your machine that will allow you to compile and interpret WebAssembly modules. These tools include:

####	WebAssembly Binary Toolkit (WABT)

The WebAssembly Binary Toolkit (pronounced “wabbit”) is a general-purpose set of command-line tools you’ll use for building, examining, and troubleshooting WebAssembly modules. Whether you’re on Windows, Mac, or Linux. First thing you’re going to need to install is CMake.5 using [this link](https://tudat.tudelft.nl/installation/setupDevMacOs.html). Installing CMake varies widely across operating systems, so you’ll need to check the instructions specific to your platform. Come back and continue with the wabt installation once you’ve verified that CMake is up and running locally. Next, follow the instructions on the wabt [GitHub repository README](https://github.com/WebAssembly/wabt/blob/main/README.md) to complete the installation.

#### Extensions in VSCode.
-	```rust-analyzer```: for autocomplete and other great features.
-	```Code-LLDB```: For debugging with LLDB (even works on Windows)
-	```WebAssembly by the WebAssembly foundation```: Allows you to disassemble and inspect .wasm binaries.

#### Cargo.

If you don’t have cargo installed follow [this link](https://doc.rust-lang.org/cargo/getting-started/installation.html) to install rust as cargo comes along with it.

### Getting Started with Rust.

We shall create a simple addition program. To get started, create a new binary Rust project with the following command: 

  ``` cargo new --bin wasmi_add```

This creates a new standalone binary Rust project that can be executed from the command line. The first step is to add a dependency on the wasmi crate to the project:
 
 ```wasmi_add/Cargo.toml```

 ``` 
[package]
name = "wasmi_add"
version = "0.1.0"
authors = ["Your Address <you@address.com>"]

[dependencies]
wasmi = "0.4.0"
```

Replace your default main.rs with the following code. 
The first thing you’ll do is load the WebAssembly module (add.wasm) into a vector of bytes (the u8 type) and create a Module
from that buffer:

  ```wasmi_add/src/main.rs```

```
extern crate wasmi;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use wasmi::{ImportsBuilder, ModuleInstance, NopExternals, RuntimeValue};
fn main() -> Result<(), Box<Error>> {
    let mut buffer = Vec::new();
    {
        let mut f = File::open("../fundamentals/add.wasm")?;
        f.read_to_end(&mut buffer)?;
    }
    let module = wasmi::Module::from_buffer(buffer)?;
```

Next, we’ll create an instance of the module. You can think of this as a “running copy” of the module, which has its own state and memory. As a host, the module instance is what you’ll interact with most of the time:

```wasmi_add/src/main.rs```

```
let instance = ModuleInstance::new(&module, &ImportsBuilder::default())
        .expect("Failed to instantiate WASM module")
        .assert_no_start();
```

This code creates a new module with a default set of imports, meaning we’re not satisfying any imports demanded by the module yet. The ```assert_no_start()``` function gives us an executable module instance that will panic if the module has a ```start()``` function. If we knew our module needed initialization, we’d call the ```run_start()``` function instead. The use of ```expect()``` is just another way of forcing a panic if we get a failing result.
Now that we’ve got a module instance, we can invoke a function. As a refresher, here’s what our ```add()``` function looked like:

```
(module
    (func $add (param $lhs i32) (param $rhs i32) (result i32)
        (i32.add
            (get_local $lhs)
            (get_local $rhs)
        )
    )
    (export "add" (func $add))
)
```

## Hosting Modules Outside the Browser

This code takes two ```i32``` parameters and returns an ```i32``` value. We execute that using the wasmi crate like so:

```wasmi_add/src/main.rs```

``` 
let mut args = Vec::<RuntimeValue>::new();
    args.push(RuntimeValue::from(42));
    args.push(RuntimeValue::from(1));

    let result: Option<RuntimeValue> = instance.invoke_export("add", &args, &mut NopExternals)?;
```
Here you call ```invoke_export()``` with the name of the exported function. This name must match and is case-sensitive. The ```RuntimeValue``` is used as a way of converting from Rust-native data types into values that can be passed onto the WebAssembly stack as function parameters. It’s an ```enum```, and as such, it’s incredibly easy to use pattern matching to extract results from, as shown in
the rest of the code from ```main.rs```:

``` wasmi_add/src/main.rs```

```
match result {
        Some(RuntimeValue::I32(v)) => {
            println!("The answer to your addition was {}", v);
        }
        Some(_) => {
            println!("Got a value of an unexpected data type");
        }
        None => {
            println!("Failed to get a result from wasm invocation");
        }
    }
    Ok(())
}
```

There are a couple of places in this code that are more verbose than they needed to be, but it helps to see how everything works in long form before
taking some shortcuts. When you run this code, you should see that it performs the addition just the way you’d expect:


``` Morgans-MacBook-Pro:wasmi_add neo$ cargo run``` 

```
   Compiling byteorder v1.4.3
   Compiling memory_units v0.3.0
   Compiling parity-wasm v0.31.3
   Compiling wasmi-validation v0.1.0
   Compiling wasmi v0.4.5
   Compiling wasmi_add v0.1.0 (/Users/neo/Documents/Outreachy/Confidential/WasmProject/wasm_example/wasmi_add)
warning: trait objects without an explicit `dyn` are deprecated
 --> src/main.rs:6:29
  |
6 | fn main() -> Result<(), Box<Error>> {
  |                             ^^^^^ help: use `dyn`: `dyn Error`
  |
  = note: `#[warn(bare_trait_objects)]` on by default
  = warning: this is accepted in the current edition (Rust 2018) but is a hard error in Rust 2021!
  = note: for more information, see issue #80165 <https://github.com/rust-lang/rust/issues/80165>

warning: `wasmi_add` (bin "wasmi_add") generated 1 warning
    Finished dev [unoptimized + debuginfo] target(s) in 10.14s
     Running `target/debug/wasmi_add`
The answer to your addition was 43
```
And just like that, you’ve created a Rust console application that hosts a WebAssembly module

### References: 

1. Fullstack Labs. Luis Hernandez [What is WebAssembly And What is it Used for?](https://www.fullstacklabs.co/blog/what-is-webassembly)
2.	Eric Elliot, “What is web assembly?” [What is WebAssembly - The dawn of a new Era](https://medium.com/javascript-scene/what-is-webassembly-the-dawn-of-a-new-era-61256ec5a8f6) June 19 2015.
3. The PragKevin Hoffman, “ Programming WebAssembly with Rust”, March 5 2019.
