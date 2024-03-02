use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{self, BufWriter, Seek, SeekFrom};
use std::time::Instant;
use bincode;
use crate::structures::tables::{Producer, Types, Consumer};


pub fn store_data_serialized(file_path: &str) -> io::Result<()> {
    let data_file:File = OpenOptions::new().append(true).create(true).open(file_path)?;
    let mut data_writer:BufWriter<File> = BufWriter::new(data_file);

    let start:Instant = Instant::now();

    for id in 1..=1_00_000{
        let data = Producer {
            id,
            name: format!("Person {}", id),
            price: 10.0 * id as f32,
            adjacency_list: HashMap::new(),
        };
        bincode::encode_into_std_write(&data, &mut data_writer, bincode::config::standard()).expect("Failed");
    }
    println!("Total write time {:?}", start.elapsed());
    println!("------------------------------------");
    Ok(())
}