pub fn parse_first_epoch(texto: &str) -> String {
    let mut en_datos = false;

    for line in texto.lines() {
        if en_datos {
            return format!("Primer epoch:\n{}", line);
        }

        if line.contains("END OF HEADER") {
            en_datos = true;
        }
    }

    "No se encontró epoch".to_string()
}