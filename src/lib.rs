use wasm_bindgen::prelude::*;
mod parser;

#[wasm_bindgen]
pub fn leer_archivo(data: &[u8]) -> String {
    parser::parse(data)
}