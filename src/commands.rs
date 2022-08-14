use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;

pub fn encode(file_path: &Path, chunk_type: &str, message: &str, output_path: &Option<PathBuf>) {
    let mut png = read_png(file_path);
    let chunk_type = ChunkType::from_str(chunk_type).expect("Failed to creat chunk type");
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(chunk);

    let output_path = match output_path {
        Some(path) => path.to_owned(),
        None => file_path.to_owned(),
    };

    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(output_path)
        .expect("Failed to open output file");

    output_file
        .write(png.as_bytes().as_slice())
        .expect("Failed to write output file");
}

pub fn decode(file_path: &Path, chunk_type: &str) {
    todo!()
}

pub fn remove(file_path: &Path, chunk_type: &str) {
    todo!()
}

pub fn print(file_path: &Path) {
    let png = read_png(file_path);
    println!("{}", png);
}

fn read_png(file_path: &Path) -> Png {
    let f = std::fs::File::open(file_path).expect("Failed to open file");
    let mut reader = std::io::BufReader::new(f);
    let mut bytes = Vec::new();

    reader
        .read_to_end(&mut bytes)
        .expect("Failed to read PNG data");

    let png = Png::try_from(&bytes[..]).expect("Failed to read PNG");

    png
}
