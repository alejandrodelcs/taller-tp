use std::path::Path;

#[derive(Debug)]
pub enum FileType {
    RinexObs,
    RinexCompressed,
    RinexZ,
    Sp3,
    Sp3Z,
    Sinex,
    SinexZ,
    Unknown,
}

pub fn detect_file_type(path: &str) -> FileType {
    let p = Path::new(path);

    let ext = p
        .extension()
        .and_then(|s| s.to_str())
        .unwrap_or("")
        .to_lowercase();

    let last = path.chars().last();

    match (ext.as_str(), last) {
        // 🛰 SP3
        ("sp3", _) => FileType::Sp3,
        ("z", _) if path.to_lowercase().ends_with(".sp3.z") => FileType::Sp3Z,

        // 📄 SINEX
        ("snx", _) => FileType::Sinex,
        ("z", _) if path.to_lowercase().ends_with(".snx.z") => FileType::SinexZ,

        // 📦 RINEX comprimido
        ("z", _) => FileType::RinexZ,

        // 📡 RINEX viejo formato
        (_, Some('o')) | (_, Some('O')) => FileType::RinexObs,
        (_, Some('d')) | (_, Some('D')) => FileType::RinexCompressed,

        _ => FileType::Unknown,
    }
}
