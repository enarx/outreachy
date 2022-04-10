#How to Build and Run the rust file
The steps are also included in the **"Basic example tutorial"**,but considers you that you
want to dive into the working with the code at once.
Again, Clone the [GitHub repository README](https://github.com/WebAssembly/wabt/blob/main/README.md) as this rust code makes use of the ../fundamentals/add file in it. 
In your terminal, type in ```cargo run```.

The result should look like this and print the sum of the 2 numbers. Inserting a few lines of the result in the terminal below:

```
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