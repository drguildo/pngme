pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub mod chunk;
pub mod chunk_type;
pub mod png;
