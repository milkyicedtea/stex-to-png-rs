use std::env;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Seek, SeekFrom};
use std::path::Path;

const GODOT_HEADER_SIZE: u64 = 32;

fn convert_file(path: &str) -> io::Result<()> {
    let input = File::open(path)?;
    let mut reader = BufReader::new(input);
    reader.seek(SeekFrom::Start(GODOT_HEADER_SIZE))?;

    let output_path = format!("{}-output.png", path);
    let output = File::create(&output_path)?;
    let mut writer = BufWriter::new(output);

    io::copy(&mut reader, &mut writer)?;

    println!("Converted: {} -> {}", path, output_path);
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: stex-to-png-rs <file1.stex> [file2.stex ...]");
        std::process::exit(1);
    }

    for path in args {
        if !Path::new(&path).exists() {
            eprintln!("File not found: {}", path);
            continue;
        }

        if let Err(err) = convert_file(&path) {
            eprintln!("Failed to convert {}: {}", path, err);
        }
    }
}
