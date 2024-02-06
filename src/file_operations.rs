use std::{io, io::Result};
use std::path::Path;
use std::fs::{OpenOptions, File};
use colored::*;
pub(crate) fn create_file() -> io::Result<()> {
    let file_path = "Quanta.db";

    if Path::new(file_path).exists() {
        println!("{}", "File already exists.".red());
    } else {
        File::create(file_path)?;
        println!("{}", "File created successfully.".red());
    }
    Ok(())
}

pub fn clear_file(file_path: &str) -> Result<()> {
    OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)?;
    println!("{}", "File cleared successfully.".red());
    Ok(())
}
