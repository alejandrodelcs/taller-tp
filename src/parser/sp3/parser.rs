pub fn parse(texto: &str) -> String {
    let mut resultado = String::new();

    for (i, line) in texto.lines().enumerate().take(50) {
        resultado.push_str(&format!("{:02}: {}\n", i, line));
    };

    resultado
}