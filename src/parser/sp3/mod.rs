pub mod parser;

pub fn parse(texto: &str) -> String {
    parser::parse(texto)
}