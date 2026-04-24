mod pipeline;
mod rinex_reader;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Uso: cargo run -- <archivo.Z>");
        return;
    }

    let input = &args[1];

    let output = pipeline::process_file(input)
        .expect("Error en pipeline");

    rinex_reader::read_rinex(&output)
        .expect("Error leyendo RINEX");
}