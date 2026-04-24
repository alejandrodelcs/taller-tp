use std::path::Path;
use std::process::Command;

pub fn process_file(input: &str) -> Result<String, String> {
    println!("Input: {}", input);

    // Caso 1: ya es .o → no hacer nada
    if input.ends_with(".o") {
        println!("⏭Archivo ya en formato RINEX (.o)");
        return Ok(input.to_string());
    }

    // Caso 2: es .d → solo Hatanaka
    if input.ends_with(".d") {
        let output = guess_output_name(input);

        if !Path::new(&output).exists() {
            println!("Ejecutando crx2rnx...");

            let status = Command::new("./crx2rnx.exe")
                .arg(input)
                .status()
                .map_err(|e| e.to_string())?;

            if !status.success() {
                return Err("Error en crx2rnx".into());
            }
        } else {
            println!("Skip crx2rnx (ya existe {})", output);
        }

        return Ok(output);
    }

    // Caso 3: es .Z → gzip + Hatanaka
    if input.ends_with(".Z") {
        let decompressed = input.trim_end_matches(".Z");
        let output = guess_output_name(decompressed);

        // gzip
        if !Path::new(decompressed).exists() {
            println!("🔧 Descomprimiendo .Z...");

            let status = Command::new("./gzip.exe")
                .arg("-d")
                .arg(input)
                .status()
                .map_err(|e| e.to_string())?;

            if !status.success() {
                return Err("Error en gzip".into());
            }
        }

        // crx2rnx
        if !Path::new(&output).exists() {
            println!("🧠 Ejecutando crx2rnx...");

            let status = Command::new("./crx2rnx.exe")
                .arg(decompressed)
                .status()
                .map_err(|e| e.to_string())?;

            if !status.success() {
                return Err("Error en crx2rnx".into());
            }
        }

        return Ok(output);
    }

    Err("Formato no soportado".into())
}

fn guess_output_name(input: &str) -> String {
    let path = Path::new(input);

    if let Some(stem) = path.file_stem() {
        let mut name = stem.to_string_lossy().to_string();
        name.push_str(".26o");
        return name;
    }

    "output.26o".to_string()
}