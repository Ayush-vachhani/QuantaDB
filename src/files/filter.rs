use std::fs::File;
use std::io;
use std::io::BufReader;
use bincode::config;
use std::fmt::Debug;
use std::time::Instant;
use bincode::config::Configuration;
use crate::files;
use crate::structures::tables::{HasId, Producer};
pub fn read_data_filtered<T: bincode::Decode + Debug + HasId>(file_path: &str) -> io::Result<()> {
    let file :File= File::open(file_path)?;
    let mut reader:BufReader<File> = BufReader::with_capacity(300_0000,file);
    let mut nodes :Vec<T>= Vec::with_capacity(100_000);
    let start:Instant = std::time::Instant::now();

    while let Ok(data) = bincode::decode_from_std_read::<T, Configuration, BufReader<File>>(&mut reader, config::standard()) {
        if data.id() % 2 == 0 {
            nodes.push(data);
        }
    }
    println!("Total read time {:?}", start.elapsed());
    println!("FILE NAME {:?}", file_path);
    files::utils::file_size(file_path).unwrap();
    // for node in nodes {
    //     println!("{:?}", node);
    // }
    println!("------------------------------------");
    Ok(())
}