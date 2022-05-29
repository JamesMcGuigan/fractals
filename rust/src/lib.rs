use wasm_bindgen::prelude::*;

mod tests;
mod colormap;

#[wasm_bindgen]
pub fn squared(x: i32) -> i32 {
    return x * x;
}
