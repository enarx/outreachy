use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn helloworld() -> String {
    String::from("Hello World using Rust!")
}