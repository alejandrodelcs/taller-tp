use wasm_bindgen::prelude::*;
mod parser;
use crate::parser::common::limpiar;

#[wasm_bindgen]
pub fn leer_archivo(data: &[u8]) -> String {
    let texto = limpiar(data);

    parser::parse(&texto.as_bytes())
}