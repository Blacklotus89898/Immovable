use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[wasm_bindgen]
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}


#[wasm_bindgen]
pub fn reverse_string(input: &str) -> String {
    input.chars().rev().collect()
}

#[wasm_bindgen]
pub fn test(a: i32, b: i32) -> i32 {
    a + b
}