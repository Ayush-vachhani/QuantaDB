use std::fs::{OpenOptions};
use std::io::{self, BufWriter, Seek, Write};
use std::time::Instant;
use bincode;
use crate::structures::tables::Person;


pub fn store_data_serialized(file_path: &str, index_file_path:&str) -> io::Result<()> {
    let data_file = OpenOptions::new().append(true).create(true).open(file_path)?;
    let mut data_writer = BufWriter::new(data_file);

    let index_file = OpenOptions::new().append(true).create(true).open(index_file_path)?;
    let mut index_writer = BufWriter::new(index_file);

    let start = Instant::now();

    for id in 1..=100_000 {
        let person = Person {
            id,
            name: format!("Person {}", id),
            age: (id * 2) ,
        };
        bincode::encode_into_std_write(&person, &mut data_writer, bincode::config::standard()).expect("Failed");

        let offset = data_writer.stream_position()?;
        if &person.age % 10 == 0 {
            bincode::encode_into_std_write(offset, &mut index_writer, bincode::config::standard()).expect("Failed on storing pointer info.");
        }
    }
    println!("Total write time {:?}", start.elapsed());
    println!("------------------------------------");
    Ok(())
}