use std::fmt::Display;

use crate::{chunk_type::ChunkType, Error, Result};

pub struct Chunk {
    chunk_type: ChunkType,
    data: Vec<u8>,
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        let (chunk_data_length_bytes, bytes) = bytes.split_at(Chunk::LENGTH_SIZE);
        let chunk_data_length = u32::from_be_bytes(chunk_data_length_bytes.try_into()?) as usize;

        // Check whether the input slice can provide as many bytes as we need.
        if (chunk_data_length + (Chunk::METADATA_SIZE - Chunk::LENGTH_SIZE)) > bytes.len() {
            return Err(Box::new(ChunkError::InputTooSmall(
                chunk_data_length,
                bytes.len(),
            )));
        }

        let (chunk_type_bytes, bytes) = bytes.split_at(Chunk::CHUNK_TYPE_SIZE);
        let chunk_type_bytes: [u8; 4] = chunk_type_bytes.try_into()?;
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;

        if !chunk_type.is_valid() {
            return Err(Box::new(ChunkError::InvalidChunkType(
                chunk_type.to_string(),
            )));
        }

        let (chunk_data, bytes) = bytes.split_at(chunk_data_length);
        let (checksum_bytes, _) = bytes.split_at(Chunk::CRC_SIZE);

        let new_chunk = Chunk::new(chunk_type, chunk_data.to_owned());

        let checksum = u32::from_be_bytes(checksum_bytes.try_into()?);
        let calculated_checksum = new_chunk.crc();

        if checksum != calculated_checksum {
            return Err(Box::new(ChunkError::InvalidCrc(
                calculated_checksum,
                checksum,
            )));
        }

        Ok(new_chunk)
    }
}

impl Display for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {} bytes", self.data().len())?;
        writeln!(f, "  Crc: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}

impl Chunk {
    pub const CHUNK_TYPE_SIZE: usize = 4;
    pub const LENGTH_SIZE: usize = 4;
    pub const CRC_SIZE: usize = 4;
    pub const METADATA_SIZE: usize = Chunk::CHUNK_TYPE_SIZE + Chunk::LENGTH_SIZE + Chunk::CRC_SIZE;

    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        Chunk { chunk_type, data }
    }
    pub fn length(&self) -> usize {
        self.data.len()
    }
    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
    pub fn crc(&self) -> u32 {
        let bytes: Vec<u8> = self
            .chunk_type
            .bytes()
            .iter()
            .cloned()
            .chain(self.data.iter().cloned())
            .collect();
        crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&bytes)
    }
    pub fn data_as_string(&self) -> Result<String> {
        let s = std::str::from_utf8(&self.data)?;
        Ok(String::from(s))
    }
    pub fn as_bytes(&self) -> Vec<u8> {
        let length_bytes = u32::to_be_bytes(self.data().len() as u32);
        let type_bytes = self.chunk_type().bytes();
        let crc_bytes = u32::to_be_bytes(self.crc());
        length_bytes
            .iter()
            .chain(type_bytes.iter())
            .chain(self.data().iter())
            .chain(crc_bytes.iter())
            .copied()
            .collect::<Vec<u8>>()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ChunkError {
    InputTooSmall(usize, usize),
    InvalidCrc(u32, u32),
    InvalidChunkType(String),
}
impl std::error::Error for ChunkError {}
impl Display for ChunkError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkError::InputTooSmall(required, available) => {
                write!(
                    f,
                    "Input size {} too small, expected {}",
                    available, required
                )
            }
            ChunkError::InvalidCrc(expected, actual) => {
                write!(f, "Invalid CRC {}, expected {}", actual, expected)
            }
            ChunkError::InvalidChunkType(chunk_type) => {
                write!(f, "Invalid chunk type {}", chunk_type)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::chunk_type::ChunkType;
    use std::str::FromStr;

    fn testing_chunk() -> Chunk {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        Chunk::try_from(chunk_data.as_ref()).unwrap()
    }

    #[test]
    fn test_new_chunk() {
        let chunk_type = ChunkType::from_str("RuSt").unwrap();
        let data = "This is where your secret message will be!"
            .as_bytes()
            .to_vec();
        let chunk = Chunk::new(chunk_type, data);
        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_chunk_length() {
        let chunk = testing_chunk();
        assert_eq!(chunk.length(), 42);
    }

    #[test]
    fn test_chunk_type() {
        let chunk = testing_chunk();
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
    }

    #[test]
    fn test_chunk_string() {
        let chunk = testing_chunk();
        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");
        assert_eq!(chunk_string, expected_chunk_string);
    }

    #[test]
    fn test_chunk_crc() {
        let chunk = testing_chunk();
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_valid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref()).unwrap();

        let chunk_string = chunk.data_as_string().unwrap();
        let expected_chunk_string = String::from("This is where your secret message will be!");

        assert_eq!(chunk.length(), 42);
        assert_eq!(chunk.chunk_type().to_string(), String::from("RuSt"));
        assert_eq!(chunk_string, expected_chunk_string);
        assert_eq!(chunk.crc(), 2882656334);
    }

    #[test]
    fn test_invalid_chunk_from_bytes() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656333;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk = Chunk::try_from(chunk_data.as_ref());

        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_try_from_with_excess_data() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .chain([0x12, 0x34, 0x56, 0x78].iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();
        assert_eq!(chunk.data(), message_bytes);
    }

    #[test]
    pub fn test_chunk_trait_impls() {
        let data_length: u32 = 42;
        let chunk_type = "RuSt".as_bytes();
        let message_bytes = "This is where your secret message will be!".as_bytes();
        let crc: u32 = 2882656334;

        let chunk_data: Vec<u8> = data_length
            .to_be_bytes()
            .iter()
            .chain(chunk_type.iter())
            .chain(message_bytes.iter())
            .chain(crc.to_be_bytes().iter())
            .copied()
            .collect();

        let chunk: Chunk = TryFrom::try_from(chunk_data.as_ref()).unwrap();

        let _chunk_string = format!("{}", chunk);
    }

    #[test]
    fn test_chunk_as_bytes() {
        let chunk = testing_chunk();
        let chunk_bytes = [
            0, 0, 0, 42, 82, 117, 83, 116, 84, 104, 105, 115, 32, 105, 115, 32, 119, 104, 101, 114,
            101, 32, 121, 111, 117, 114, 32, 115, 101, 99, 114, 101, 116, 32, 109, 101, 115, 115,
            97, 103, 101, 32, 119, 105, 108, 108, 32, 98, 101, 33, 171, 209, 216, 78,
        ];
        assert_eq!(chunk.as_bytes(), chunk_bytes);
    }
}
