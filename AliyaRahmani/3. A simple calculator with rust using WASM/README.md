# A simple calculator with rust using WASM
![Image description](https://www.wasm.builders/images/PulTEJkg_yKo5brFth0UtkDTMlMeD2jWztdOJ40d81E/s:1000:420/mb:500000/ar:1/aHR0cHM6Ly93d3cu/d2FzbS5idWlsZGVy/cy9yZW1vdGVpbWFn/ZXMvdXBsb2Fkcy9h/cnRpY2xlcy9pcGhj/ZnYzZGY2cGZpd3Ux/NWViOS5qcGc)

## WebAssembly with rust
  Rust is the most suitable language for WebAssembly. This is because of the small runtime, excellent efficiency, and a memory-safe language.
  Today, we're going to look into running a simple calculator rust program using wasmtime.

### Installing rust in ubuntu

The first step is to setup rust in your local OS, I'm using ubuntu so here's the command for that:
`curl https://sh.rustup.rs -sSf | sh`

### Install Wasmtime

`curl https://wasmtime.dev/install.sh -sSf | bash`

### Install the WebAssembly Rust toolchain:​

`rustup target install wasm32-wasi`

## Rust Code​

We need to create a rust application with the command:

```
cargo new example
cd example/src
```

![Image description](https://www.wasm.builders/images/s_jNaYQL-a9EqhAAJ83x2cFzzFdcHm6RkkWVZquJfa0/w:880/mb:500000/ar:1/aHR0cHM6Ly93d3cu/d2FzbS5idWlsZGVy/cy9yZW1vdGVpbWFn/ZXMvdXBsb2Fkcy9h/cnRpY2xlcy9sZTRr/Zzg3NzdidHNsMGx5/eGg5YS5wbmc)
For a simple calculator, add the following code to the main.rs file:

```
use std::io::{stdin, stdout, Write};
fn read(input: &mut String) {
    stdout().flush()
        .expect("failed to flush");
    stdin().read_line(input)
        .expect("failed to read");
}
fn main() {
    println!("welcome to engineer man's calculator!");
    println!("---------");
loop {
        let mut num1 = String::new();
        let mut num2 = String::new();
        let mut operator = String::new();
print!("what is the first number?: ");
        read(&mut num1);
print!("what is the second number?: ");
        read(&mut num2);
print!("what operation would you like to do? [+-*/]: ");
        read(&mut operator);
let num1: f32 = num1.trim().parse().unwrap();
        let num2: f32 = num2.trim().parse().unwrap();
        let operator: char = operator.trim().chars().next().unwrap();
let operators = String::from("+-*/");
if !operators.contains(operator) {
            println!("unknown operator");
            continue;
        }
let result = match operator {
            '+' => num1 + num2,
            '-' => num1 - num2,
            '*' => num1 * num2,
            '/' => num1 / num2,
            _ => panic!("error in operator")
        };
println!("the result of {} {} {} = {}", num1, operator, num2, result);
    }
}
```

![Image description](https://www.wasm.builders/images/Blh-SEqEwtQqoEEEC1MtuDmOHxI2yIm5ENNt4-cHTMA/w:880/mb:500000/ar:1/aHR0cHM6Ly93d3cu/d2FzbS5idWlsZGVy/cy9yZW1vdGVpbWFn/ZXMvdXBsb2Fkcy9h/cnRpY2xlcy9iYjVi/OGt6cDY5eDQ0a2t2/Yzdmbi5wbmc)
## Compiling Rust Code​
Compile using cargo to verify the code:

```
cargo build
cargo run
```

![Image description](https://www.wasm.builders/images/a0ixuwx656yfSsVokjNYRooa4mXDt17ciOZy9w3An8Q/w:880/mb:500000/ar:1/aHR0cHM6Ly93d3cu/d2FzbS5idWlsZGVy/cy9yZW1vdGVpbWFn/ZXMvdXBsb2Fkcy9h/cnRpY2xlcy96NHpu/ZDN6aDJrNnV6Zmp2/eWx2di5wbmc)
### Compile to wasm

```
rustc main.rs --target wasm32-wasi3.
```

## Run the program:

```
wasmtime main.wasm
```

![Image description](https://www.wasm.builders/images/_8b8EswwyyDN6ytsStDGwGO1oXK8fgmh5gSNszkXuF0/w:880/mb:500000/ar:1/aHR0cHM6Ly93d3cu/d2FzbS5idWlsZGVy/cy9yZW1vdGVpbWFn/ZXMvdXBsb2Fkcy9h/cnRpY2xlcy9pMmI3/ajl5YW9lMjNmZ3B4/aWVhNy5wbmc)
Yaaay! We finally executed our calculator in rust using WASM!
