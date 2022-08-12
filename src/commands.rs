use std::io::Read;
use std::path::Path;

use crate::png::Png;

pub fn encode(file_path: &Path, chunk_type: &str, message: &str, output_file: Option<&Path>) {
    todo!()
}

pub fn decode(file_path: &Path, chunk_type: &str) {
    todo!()
}

pub fn remove(file_path: &Path, chunk_type: &str) {
    todo!()
}

pub fn print(file_path: &Path) {
    let bytes = read_file(file_path);
    let png = Png::try_from(&bytes[..]).expect("Failed to read PNG");
    println!("{}", png);
}

fn read_file(file_path: &Path) -> Vec<u8> {
    let f = std::fs::File::open(file_path).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader
        .read_to_end(&mut buffer)
        .expect("Failed to read PNG data");

    buffer
}
