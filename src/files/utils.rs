use std::fs;
use std::fs::File;
use std::io::{Result};

use std::fs::OpenOptions;
use std::io::{self, ErrorKind};

pub fn create_quanta_db() -> io::Result<()> {
    match OpenOptions::new().write(true).create_new(true).open("Quanta.db") {
        Ok(_) => {
            Ok(())
        },
        Err(ref e) if e.kind() == ErrorKind::AlreadyExists => {
            Ok(())
        },
        Err(e) => {
            println!("Failed to create Quanta.db file: {:?}", e);
            Err(e)
        }
    }
}
pub fn clear_file(file_path: &str) -> Result<()> {
    let _file = File::create(file_path)?;
    println!("File cleared successfully.");
    Ok(())
}

pub fn file_size(file_path: &str) -> Result<()>{
    let metadata = fs::metadata(file_path)?;
    let file_size_in_kb = metadata.len() as f64 / 1024.0;
    let file_size_in_bytes = metadata.len();
    println!("File size in bytes: {} bytes", file_size_in_bytes);
    println!("File size in kb's: {:.2} KB", file_size_in_kb);
    Ok(())
}
