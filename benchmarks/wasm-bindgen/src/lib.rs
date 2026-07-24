use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn i32_identity(value: i32) -> i32 {
	value
}
