use project_fiuba::pipeline;
use project_fiuba::reader;

fn main() {
    let input = std::env::args().nth(1).expect("Uso: cargo run archivo");

    let path = pipeline::process_file(&input).expect("Error en pipeline");

    reader::read_file(&path).expect("Error leyendo archivo");
}
