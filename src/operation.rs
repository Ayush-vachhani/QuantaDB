use std::collections::HashMap;
use std::time::{Duration, Instant};
use colored::Colorize;
use crate::file_operations::{clear_file, create_file};
use crate::Node;
use crate::read_file::read_nodes;
use crate::write_file::store_node;

pub fn operation(num_nodes:u64) -> std::io::Result<((Duration, Duration))> {
    create_file().expect(&*"Failed creation".red());

    let mut nodes:Vec<Node> = Vec::new();

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
    let duration_write = start.elapsed();

    let start = Instant::now();
    read_nodes()?;
    let duration_read = start.elapsed();

    clear_file("Quanta.db")?;
    Ok((duration_read, duration_write))
}