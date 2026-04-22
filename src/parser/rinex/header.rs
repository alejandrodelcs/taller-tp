pub fn parse_header(texto: &str) -> String {
    let mut marker = "No encontrado";
    let mut position = "No encontrada";
    let mut interval = "No encontrado";

    for line in texto.lines() {
        if line.contains("MARKER NAME") {
            marker = line.trim();
        }

        if line.contains("APPROX POSITION XYZ") {
            position = line.trim();
        }

        if line.contains("INTERVAL") {
            interval = line.trim();
        }

        if line.contains("END OF HEADER") {
            break;
        }
    }

    format!(
        "Header:\nMarker: {}\nPosición: {}\nIntervalo: {}",
        marker, position, interval
    )
}