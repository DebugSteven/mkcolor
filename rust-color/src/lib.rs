use wasm_bindgen::prelude::*;

// javascript function
#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

// rust using javascript function
#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}
