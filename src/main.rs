use std::io::{self, BufWriter};
use tose_converter::ToseConverter;

fn main() -> io::Result<()> {
    // Create converter and process stdin to stdout
    let converter = ToseConverter::new();
    let stdin = io::stdin();
    let stdout = io::stdout();
    let writer = BufWriter::new(stdout.lock());

    converter.convert(stdin.lock(), writer)?;

    Ok(())
}
