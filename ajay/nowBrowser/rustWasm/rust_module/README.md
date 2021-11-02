
# Module
A logical group of code is called a Module. Multiple modules are compiled into a unit called crate where crate is a compilation unit in Rust.
Modulea in Rust are private by default.

### Declaration Syntax

```
mod private_module{

    fn private_module_private_function(){
        println!("private_module_private_function get called\n");
    }
    pub fn private_module_public_function(){
        private_module_private_function();
        println!("private_module_public_function get called\n");
    }
}


pub mod public_module{

    fn public_module_private_function(){
        println!("public_module_private_function get called\n");
    }
    pub fn public_module_public_function(){
        public_module_private_function();
        println!("public_module_public_function get called\n");
    }
}
```
### Syntax for accessing Module's functions
In Rust a Module can be private or public, similarily functions inside those modules also can be private and publice as shown in Declaration, while private functions can only be accessed by a public function which belong to same module say parent of both public and private module must be same. accessing Syntax are as such:

- Method I of accessing functions
```
println!("Calling public Mudules functions");
public_module_public_function();
println!("Calling Private Modules Functions");
private_module_public_function();
```
- Method II of accessing functions
```
println!("Calling public Mudules functions");
public_module::public_module_public_function();
println!("Calling Private Modules Functions");
private_module::private_module_public_function();
```
## Commands to be perform
- ### Project creation
```
cargo new --bin rust_module
```
- ### Nevigate inside Project
```
cd rust_module
```
- ### Project building/compilation
```
cargo wasi build
```
- ### Project execution
```
wasmtime target/wasm32-wasi/debug/rust_module.wasm
```
