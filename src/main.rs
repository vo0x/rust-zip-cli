mod compressor;
use std::env;
use std::error::Error;
use compressor::gzip::{compress_gzip, extract_gzip};
use compressor::zip::{zip, extract_zip};

fn print_usage() {
    eprintln!("Usage:");
    eprintln!("  compress <input> <output>     - Compress a file (output should end with .gz or .zip)");
    eprintln!("  extract <input> <output>      - Extract a file (input should be .gz or .zip)");
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 4 {
        print_usage();
        return Ok(());
    }

    let command = &args[1];
    let input = &args[2];
    let output = &args[3];

    match command.as_str() {
        "compress" => {
            if output.ends_with(".gz") {
                compress_gzip(input, output)?;
            } else if output.ends_with(".zip") {
                zip(input, output)?;
            } else {
                eprintln!("Unsupported output format. Use .gz or .zip");
            }
        }
        "extract" => {
            if input.ends_with(".gz") {
                extract_gzip(input, output)?;
            } else if input.ends_with(".zip") {
                extract_zip(input, output)?;
            } else {
                eprintln!("Unsupported input format. Use .gz or .zip");
            }
        }
        _ => {
            print_usage();
        }
    }

    Ok(())
}
