use bincode::{Decode, Encode};
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Decode, Encode, Debug)]
pub struct Person {
    pub(crate) id: i32,
    pub(crate) name: String,
    pub(crate) age: i32,
}
#[derive(Serialize, Deserialize, Decode, Encode, Debug)]
pub struct Product {
    id: i32,
    name: String,
    price: f32,
}
