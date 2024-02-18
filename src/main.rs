mod operation;
mod file_operations;
mod read_file;
mod write_file;

use operation::operation;

use std::collections::HashMap;
use std::io::{ Result };
use bincode::{Decode, Encode};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Decode, Encode)]
pub struct Node {
    id: u64,
    properties: HashMap<String, String>,
}

fn main() {
    for i in 0..1 {
        let mut sample = operation(i).unwrap().expect("Idk what happened.");
        println!("{}", sample);
    }
}

