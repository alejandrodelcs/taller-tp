pub fn parse(texto: &str) -> String {
    let mut resultado = String::new();

    for line in texto.lines().take(20) {
        if line.starts_with('*') {
            resultado.push_str(&format!("Epoch: {}\n", line));
        }

        if line.starts_with('P') {
            resultado.push_str(&format!("Sat: {}\n", line));
        }
    }

    resultado
}