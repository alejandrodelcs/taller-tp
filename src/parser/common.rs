pub fn limpiar(data: &[u8]) -> String {
    let data_limpia: Vec<u8> = data.iter()
        .filter(|&&b| b != b'\r')
        .copied()
        .collect();

    String::from_utf8_lossy(&data_limpia).to_string()
}