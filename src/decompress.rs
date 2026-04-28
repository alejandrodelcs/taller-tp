use std::path::Path;
use std::process::Command;

pub fn decompress_z(input: &str) -> Result<String, String> {
    let output = input.trim_end_matches(".Z");

    if !Path::new(output).exists() {
        println!("🔧 Descomprimiendo {}", input);

        let status = Command::new("./gzip.exe")
            .arg("-d")
            .arg(input)
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err("Error en gzip".into());
        }
    }

    Ok(output.to_string())
}
