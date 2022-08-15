use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use pngme::chunk::Chunk;
use pngme::chunk_type::ChunkType;
use pngme::png::Png;

pub fn encode(file_path: &Path, chunk_type: &str, message: &str, output_path: &Option<PathBuf>) {
    let mut png = read_png(file_path);
    let chunk_type = ChunkType::from_str(chunk_type).expect("Failed to creat chunk type");
    let chunk = Chunk::new(chunk_type, message.as_bytes().to_vec());
    png.append_chunk(chunk);

    let output_path = match output_path {
        Some(path) => path.to_owned(),
        None => file_path.to_owned(),
    };

    write_png(&output_path, &png);
}

pub fn decode(file_path: &Path, chunk_type: &str) {
    let png = read_png(file_path);
    let chunk = png.chunk_by_type(chunk_type).expect("Failed to find chunk");
    let decoded_chunk = chunk.data_as_string().expect("Failed to decode chunk");
    println!("{}", decoded_chunk);
}

pub fn remove(file_path: &Path, chunk_type: &str) {
    let mut png = read_png(file_path);
    png.remove_chunk(chunk_type).expect("Failed to remove chunk");
    write_png(file_path, &png);
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

fn write_png(output_path: &Path, png: &Png) {
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
