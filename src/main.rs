mod file_operations;
mod read_file;
mod write_file;

use file_operations::{create_file, clear_file};
use read_file::read_nodes;
use write_file::store_node;
use std::time::Instant;

use std::collections::HashMap;
use std::fmt::Display;
use std::io::{ Result };
use bincode::{Decode, Encode};
use colored::Colorize;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Decode, Encode)]
pub struct Node {
    id: u64,
    properties: HashMap<String, String>,
}

fn main() -> Result<()> {
    create_file().expect(&*"Failed creation".red());

    let mut nodes = Vec::new();
    let num_nodes = 100_000;

    for i in 0..num_nodes {
        let mut properties = HashMap::new();
        properties.insert("name".to_string(), i.to_string());

        nodes.push(Node {
            id: i,
            properties,
        });
    }

    let start = Instant::now();
    store_node(&nodes)?;
    let duration = start.elapsed();
    println!("{}", format!("Time taken to store {num_nodes} nodes: {duration}").yellow());

    let start = Instant::now();
    read_nodes()?;
    let duration_read = start.elapsed();
    println!("Time taken to read nodes: {:?}", duration_read);

    println!("Time ratio: {:?}", duration_read.as_secs_f64() / duration.as_secs_f64());

    clear_file("Quanta.db")?;
    Ok(())
}