use crate::utils;
use std::path::Path;
use std::process::Command;

pub fn run_crx2rnx(input: &str) -> Result<String, String> {
    let output = utils::guess_output_name(input);

    if !Path::new(&output).exists() {
        println!("Ejecutando crx2rnx...");

        let status = Command::new("./crx2rnx.exe")
            .arg(input)
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err("Error en crx2rnx".into());
        }
    }

    Ok(output)
}
