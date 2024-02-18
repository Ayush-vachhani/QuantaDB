use std::fs::File;
use std::io::{self, BufReader};
use crate::Node;
use bincode::config;

pub fn read_nodes() -> io::Result<Vec<Node>> {
    let file = File::open("Quanta.db")?;
    let mut reader = BufReader::new(file);

    let nodes: Vec<Node> = bincode::decode_from_std_read(&mut reader, config::standard())
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;
    // println!("Total nodes read: {}", nodes.len());
    // println!("{:?}", nodes);
    Ok(nodes)
}
