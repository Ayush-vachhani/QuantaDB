use std::fs::File;
use std::io;
use std::io::BufReader;
use bincode::config;
use std::fmt::Debug;
use crate::files;

pub fn read_data_deserialized<T: bincode::Decode + Debug>(file_path: &str) -> io::Result<()> {
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let start = std::time::Instant::now();
    let mut nodes :Vec<T>= Vec::new();


    while let Ok(person) = bincode::decode_from_std_read(&mut reader, config::standard()) {
        nodes.push(person);
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