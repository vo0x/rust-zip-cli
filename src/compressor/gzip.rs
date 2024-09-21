use flate2::Compression;
use flate2::write::GzEncoder;
use flate2::read::GzDecoder;
use std::io::{self, Read};
use std::fs::File;

use std::u64;

pub fn compress_gzip(file : &str,target : &str) -> io::Result<()>{

    let input_file = File::open(&file)?;
    let output_file = File::create(&target)?;

    let mut encoder = GzEncoder::new(output_file, Compression::default());

    io::copy(&mut input_file.take(u64::MAX), &mut encoder)?;
    encoder.finish()?;

    Ok(()) 
}

pub fn extract_gzip(file : &str, target : &str) -> io::Result<()>{
    let compressed = File::open(&file)?;
    let mut decompressed_file = File::create(&target)?;

    let mut decoder = GzDecoder::new(compressed);

    io::copy(&mut decoder, &mut decompressed_file)?;

    Ok(())
}