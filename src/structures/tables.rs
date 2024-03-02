use std::collections::HashMap;
use bincode::{Decode, Encode};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Decode, Encode, Debug)]
pub enum Types {
    Consumer,
    Producer,
}
#[derive(Serialize, Deserialize, Decode, Encode, Debug)]
pub struct Producer {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) price: f32,
    pub(crate) adjacency_list: HashMap<String, Vec<(Types, u64)>>,
}
#[derive(Serialize, Deserialize, Decode, Encode, Debug)]
pub struct Consumer {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) price: f32,
}
