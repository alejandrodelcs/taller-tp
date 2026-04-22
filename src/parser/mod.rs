pub mod rinex;
pub mod sp3;
pub mod common;

pub fn parse(data: &[u8]) -> String {
    let texto = common::limpiar(data);

    if texto.contains("RINEX") {
        rinex::parse(&texto)
    } else if texto.starts_with("#cP") || texto.starts_with("*") {
        sp3::parse(&texto)
    } else {
        "Formato no reconocido".to_string()
    }
}