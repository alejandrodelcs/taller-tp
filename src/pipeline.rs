use crate::decompress::decompress_z;
use crate::filetype::{FileType, detect_file_type};
use crate::gzip::run_crx2rnx;

pub fn process_file(input: &str) -> Result<String, String> {
    println!("📦 Input: {}", input);

    match detect_file_type(input) {
        FileType::RinexObs => {
            println!("⏭ Ya es RINEX (.o)");
            Ok(input.to_string())
        }

        FileType::RinexCompressed => run_crx2rnx(input),

        FileType::RinexZ => {
            let d = decompress_z(input)?;
            run_crx2rnx(&d)
        }

        // 🛰 SP3
        FileType::Sp3 => {
            println!("🛰 SP3 listo");
            Ok(input.to_string())
        }

        FileType::Sp3Z => {
            println!("🔧 Descomprimiendo SP3 (.Z)...");
            let d = decompress_z(input)?;
            Ok(d)
        }

        FileType::Sinex => {
            println!("📄 SINEX detectado");
            Ok(input.to_string())
        }

        FileType::SinexZ => {
            let d = decompress_z(input)?;
            Ok(d)
        }

        FileType::Unknown => Err("Formato no soportado".into()),
    }
}
