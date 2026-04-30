use crate::filetype::{FileType, detect_file_type};
use crate::rinex_reader::read_rinex;
use crate::sinex_reader::read_sinex;
use crate::sp3_reader::read_sp3;

pub fn read_file(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    match detect_file_type(path) {
        FileType::RinexObs => {
            let _data = read_rinex(path)?;
            //println!("📡 {} observaciones", data.);
        }

        FileType::Sp3 => {
            let data = read_sp3(path)?;
            println!("🛰 {} registros SP3", data.len());

            //Esto es solo una prueba
            for rec in data.iter().take(5) {
                println!("{:?}", rec);
            }
        }

        FileType::Sinex => {
            let _ = read_sinex(path);
        }

        _ => println!("⚠️ Tipo no manejado para lectura"),
    }

    Ok(())
}
