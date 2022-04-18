extern crate wasm_bindgen;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);

    type HTMLDocument;
    type Element;

    static document: HTMLDocument;

    #[wasm_bindgen(method)]
    fn createElement(this: &HTMLDocument, tagName: &str) -> Element;

    #[wasm_bindgen(method, getter)]
    fn body(this: &HTMLDocument) -> Element;

    #[wasm_bindgen(method, js_name = appendChild)]
    fn append(this: &Element, item: Element);

    #[wasm_bindgen(method, setter = innerHTML)]
    fn set_inner(this: &Element, html: &str);
}

#[wasm_bindgen]
pub fn run_alert(item: &str) {
    alert(&format!("This is WASM and {}", item));
}

#[wasm_bindgen]
pub fn create_stuff() {
    let div = document.createElement("div");
    let p = document.createElement("p");

    p.set_inner("Hello from WASM in Rust!");
    div.append(p);

    document.body().append(div);
}
