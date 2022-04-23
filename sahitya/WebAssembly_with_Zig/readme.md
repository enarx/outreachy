## Enviroment setup

TO compline this demo, you must install these packages listed below.

# zig

Install zig. 

### Install zig on Fedora 34 Using dnf 
```bash
sudo dnf makecache --refresh
```
After updating yum database, We can install zig using dnf by running the following command:

```bash
sudo dnf -y install zig
```

Note : if you are using other operating system Go to [ziglang.org](https://ziglang.org/download/) and follow the instructions.


# Wasmtime
You will find wasmtime at wasmtime.dev


## Zig Code

```
const std = @import("std");

fn fibonacci(index: u32) u32 {
    if (index < 2) return index;
    return fibonacci(index - 1) + fibonacci(index - 2);
}

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    var x: u32 = 7;
    
    try stdout.print("fibonacci of {d} ", .{x});
    try stdout.print("is: {d} \n ", .{fibonacci(x)}  );
}
```


# Compiling zig code

1. compile using zig

```bash
zig run main.zig
```

2. compile to WASM using the following command: 

```bash
zig build-exe main.zig -target wasm32-wasi
```

3. file main.wasm
```bash
file main.wasm
```
4. wasm runtime
```bash
wasmtime main.wasm
```

![image](https://user-images.githubusercontent.com/37513489/163274402-f90c4c46-add3-4777-bcec-65b3edb40e75.png)
