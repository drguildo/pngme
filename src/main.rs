mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::{
    fs::File,
    io::{BufReader, Read},
    path::{Path, PathBuf},
};

use clap::{Parser, Subcommand};
use png::Png;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(propagate_version = true)]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Encode {
        file_path: PathBuf,
        chunk_type: String,
        message: String,
        output_file: Option<PathBuf>,
    },
    Decode {
        file_path: PathBuf,
        chunk_type: String,
    },
    Remove {
        file_path: PathBuf,
        chunk_type: String,
    },
    Print {
        file_path: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Encode {
            file_path,
            chunk_type,
            message,
            output_file,
        } => todo!(),
        Commands::Decode {
            file_path,
            chunk_type,
        } => todo!(),
        Commands::Remove {
            file_path,
            chunk_type,
        } => todo!(),
        Commands::Print { file_path } => {
            let bytes = read_file(file_path);
            let png = Png::try_from(&bytes[..]).expect("Failed to read PNG");
            println!("{}", png);
        }
    }
}

fn read_file(file_path: &Path) -> Vec<u8> {
    let f = File::open(file_path).expect("Failed to open file");
    let mut reader = BufReader::new(f);
    let mut buffer = Vec::new();

    // Read file into vector.
    reader.read_to_end(&mut buffer).expect("Failed to read PNG data");

    buffer
}
