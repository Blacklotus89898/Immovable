pub mod module1;
pub mod module2;
pub mod terminal_app; // Add here to generate docs for the terminal_app module

mod webasm; // This is the module that will be compiled to WebAssembly

use wasm_bindgen::prelude::*;
// pub use webasm::multiply;
// pub use webasm::reverse_string;
// pub use webasm::greet;


#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
