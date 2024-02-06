use std::fs::OpenOptions;
use std::io::{self, Write, BufWriter};
use bincode;
use crate::Node;
pub fn store_node(node: &Vec<Node>) -> io::Result<String> {

    let file = OpenOptions::new().append(true).create(true).open("Quanta.db")?;
    let mut writer = BufWriter::new(file);
    bincode::encode_into_std_write(node, &mut writer, bincode::config::standard()).expect("Write failed");
    writer.flush()?;
    Ok(("Nodes stored successfully.").parse().unwrap())
}
