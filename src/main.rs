mod args;
mod chunk;
mod chunk_type;
mod commands;
mod png;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
        output_path: Option<PathBuf>,
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
            output_path,
        } => {
            commands::encode(file_path, chunk_type, message, output_path);
        }
        Commands::Decode {
            file_path,
            chunk_type,
        } => {
            commands::decode(file_path, chunk_type);
        },
        Commands::Remove {
            file_path,
            chunk_type,
        } => todo!(),
        Commands::Print { file_path } => {
            commands::print(file_path);
        }
    }
}
