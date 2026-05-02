use rinex::Rinex;
use rinex::prelude::Observable;
use serde::Serialize;
use serde_json;
use std::fs::File;
use std::io::Write;


#[derive(Debug, Serialize)]
pub struct Record {
    pub epoch: String,
    pub sv: String,
    pub obs_code: String,
    pub value: f64,
}


pub fn parse_observable(obs: &Observable) -> String {
    match obs {
        Observable::PhaseRange(code)
        | Observable::PseudoRange(code)
        | Observable::Doppler(code)
        | Observable::SSI(code) => code.to_string(),

        _ => format!("{:?}", obs),
    }
}



pub fn read_rinex(path: &str) -> Result<Vec<Record>, Box<dyn std::error::Error>> {
    let rinex = Rinex::from_file(path)?;

    let mut records = Vec::new();

    for (key, observations) in rinex.observations_iter() {
        for signal in &observations.signals {
            let code = parse_observable(&signal.observable);

            records.push(Record {
                 epoch: format!("{:?}", key.epoch),
                sv: format!("{:?}", signal.sv),
                obs_code: code,
                value: signal.value,
            });
        }
    }
    dbg!(&records[0]);

    let json = serde_json::to_string_pretty(&records)?;
    let mut file = File::create("output.json")?;
    file.write_all(json.as_bytes())?;
    
    Ok(records)
}
