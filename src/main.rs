use crate::structures::tables::Person;

mod structures;
mod files;

fn main() {
    let file_name = "Quanta.db";
    let index_file_name = "QuantaIndex.db";
    files::utils::create_quanta_db().unwrap(); //create file
    files::write::store_data_serialized(file_name,index_file_name).expect("File not found in path"); // Write data to file
    files::read::read_data_deserialized::<u64>(index_file_name).expect("Error reading data"); // Read data from file
    files::read::read_data_deserialized::<Person>(file_name).expect("Error reading data"); // Read indices from file
    files::utils::clear_file(file_name).expect("Error clearing file"); // Clear file
    files::utils::clear_file(index_file_name).expect("Error clearing file"); // Clear file
}