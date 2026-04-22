pub mod header;
pub mod epoch;

pub fn parse(texto: &str) -> String {
    let header = header::parse_header(texto);
    let epoch = epoch::parse_first_epoch(texto);

    format!(
        "RINEX\n\n{}\n\n{}",
        header,
        epoch
    )
}