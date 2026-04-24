use rinex::Rinex;

pub fn read_rinex(path: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📡 Leyendo RINEX: {}", path);

    let rinex = Rinex::from_file(path)?;

    println!("Tipo: {:?}", rinex.header.rinex_type);

    for (key, signal) in rinex.signal_observations_iter().take(10) {
        println!("{:?} → {:?}", key.epoch, signal);
    }
    Ok(())
}
