use std::env;
use std::io::{self, BufWriter};
use tose_converter::ToseConverter;

fn main() -> io::Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <ENTITY_NAME> [FIELD_1 FIELD_2 ... FIELD_N]", args[0]);
        eprintln!("Error: Missing required arguments.");
        std::process::exit(1);
    }

    let entity_name = args[1].clone();
    let fields: Vec<String> = args[2..].iter().cloned().collect();

    // Create converter and process stdin to stdout
    let converter = ToseConverter::new(entity_name, fields);
    let stdin = io::stdin();
    let stdout = io::stdout();
    let writer = BufWriter::new(stdout.lock());

    converter.convert(stdin.lock(), writer)?;

    Ok(())
}
