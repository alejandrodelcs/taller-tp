use sp3::SP3;

#[derive(Debug)]
pub struct OrbitRecord {
    pub sv: String,
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

pub fn read_sp3(path: &str) -> Result<Vec<OrbitRecord>, Box<dyn std::error::Error>> {
    let sp3 = SP3::from_file(path)?;

    let mut result = Vec::new();

    for (_epoch, sv, _predicted, _maneuver, (x, y, z)) in sp3.satellites_position_km_iter() {
        result.push(OrbitRecord {
            sv: sv.to_string(),
            x,
            y,
            z,
        });
    }

    Ok(result)
}
