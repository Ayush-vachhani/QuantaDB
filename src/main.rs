use crate::structures::tables::{Producer};
mod structures;
mod files;

fn main() {
    let producer_file = "Producer.db";
    let consumer_file = "Consumer.db";
    let produces_index_file  = "ProducerIndex.db";
    let files = vec![producer_file, consumer_file, produces_index_file];
    files::utils::create_quanta_db(&files).unwrap();

    files::write::store_data_serialized(producer_file).expect("File not found in path");
    // files::filter::read_data_filtered::<Producer>(producer_file).expect("Error reading data");
    files::read::read_data_deserialized::<Producer>(producer_file).expect("Error reading data");
    files::utils::clear_files(files).expect("Error getting file size");
}