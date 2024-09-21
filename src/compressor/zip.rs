use zip::ZipWriter;
use zip::ZipArchive;
use zip::write::FileOptions;
use std::fs::File;
use std::io;
use std::path::Path;


pub fn zip(file : &str, target : &str) -> io::Result<()>{
    let mut input_file = File::open(file)?;
    let output_file = File::create(target)?;

    let mut zip = ZipWriter::new(output_file);
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);

    zip.start_file(file, options)?;
    
    io::copy(&mut input_file, &mut zip)?;
    zip.finish()?;

    Ok(())
}

pub fn extract_zip(file : &str, target : &str) -> io::Result<()>{
    let file = File::open(&file)?;
    let mut archive = ZipArchive::new(&file)?;
    
    for i in 0..archive.len() {
        let mut zip_file = archive.by_index(i)?;
        let output_path = Path::new(target).join(zip_file.name());
        
        if zip_file.name().ends_with('/') {
            
            std::fs::create_dir_all(&output_path)?;
        } else {
            
            let mut output_file = File::create(&output_path)?;
            io::copy(&mut zip_file, &mut output_file)?;
        }
    }

    Ok(())
}
