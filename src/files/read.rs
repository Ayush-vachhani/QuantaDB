use std::fs::File;
use std::io;
use std::io::BufReader;
use bincode::config;
use std::fmt::Debug;
use bincode::config::Configuration;
use crate::files;

pub fn read_data_deserialized<T: bincode::Decode + Debug>(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::with_capacity(300_0000,file);
    let mut nodes :Vec<T>= Vec::with_capacity(100_000);
    let start = std::time::Instant::now();

    while let Ok(data) = bincode::decode_from_std_read::<T, Configuration, BufReader<File>>(&mut reader, config::standard()) {
        nodes.push(data);
    }
    println!("Total read time {:?}", start.elapsed());
    println!("FILE NAME {:?}", file_path);
    files::utils::file_size(file_path).unwrap();
    for node in nodes {
        println!("{:?}", node);
    }
    println!("------------------------------------");
    Ok(())
}