use std::{fmt::Display, str::FromStr};

use crate::{Error, Result};

#[derive(Debug, PartialEq, Eq)]
pub struct ChunkType([u8; 4]);

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if bytes.iter().any(|b| !b.is_ascii_alphabetic()) {
            return Err(Box::new(ChunkTypeError::InvalidByteArray));
        }

        Ok(ChunkType(bytes))
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err(Box::new(ChunkTypeError::InvalidString));
        }

        let bytes = s.as_bytes();
        let mut chunk_type: [u8; 4] = [0; 4];
        chunk_type[..4].copy_from_slice(&bytes[..4]);
        ChunkType::try_from(chunk_type)
    }
}

impl Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = std::str::from_utf8(&self.0).map_err(|_e| std::fmt::Error)?;
        std::fmt::Display::fmt(s, f)
    }
}

impl ChunkType {
    pub fn bytes(&self) -> [u8; 4] {
        self.0
    }
    pub fn is_valid(&self) -> bool {
        self.0.iter().all(|b| b.is_ascii_alphabetic()) && self.is_reserved_bit_valid()
    }
    pub fn is_critical(&self) -> bool {
        self.0[0].is_ascii_uppercase()
    }
    pub fn is_public(&self) -> bool {
        self.0[1].is_ascii_uppercase()
    }
    pub fn is_reserved_bit_valid(&self) -> bool {
        self.0[2].is_ascii_uppercase()
    }
    pub fn is_safe_to_copy(&self) -> bool {
        self.0[3].is_ascii_lowercase()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ChunkTypeError {
    InvalidByteArray,
    InvalidString,
}
impl std::error::Error for ChunkTypeError {}
impl Display for ChunkTypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChunkTypeError::InvalidByteArray => write!(f, "Invalid byte array"),
            ChunkTypeError::InvalidString => write!(f, "Invalid string"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;
    use std::str::FromStr;

    #[test]
    pub fn test_chunk_type_from_bytes() {
        let expected = [82, 117, 83, 116];
        let actual = ChunkType::try_from([82, 117, 83, 116]).unwrap();

        assert_eq!(expected, actual.bytes());
    }

    #[test]
    pub fn test_chunk_type_from_str() {
        let expected = ChunkType::try_from([82, 117, 83, 116]).unwrap();
        let actual = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(expected, actual);
    }

    #[test]
    pub fn test_chunk_type_is_critical() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_not_critical() {
        let chunk = ChunkType::from_str("ruSt").unwrap();
        assert!(!chunk.is_critical());
    }

    #[test]
    pub fn test_chunk_type_is_public() {
        let chunk = ChunkType::from_str("RUSt").unwrap();
        assert!(chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_not_public() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(!chunk.is_public());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_reserved_bit_invalid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_reserved_bit_valid());
    }

    #[test]
    pub fn test_chunk_type_is_safe_to_copy() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_chunk_type_is_unsafe_to_copy() {
        let chunk = ChunkType::from_str("RuST").unwrap();
        assert!(!chunk.is_safe_to_copy());
    }

    #[test]
    pub fn test_valid_chunk_is_valid() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert!(chunk.is_valid());
    }

    #[test]
    pub fn test_invalid_chunk_is_valid() {
        let chunk = ChunkType::from_str("Rust").unwrap();
        assert!(!chunk.is_valid());

        let chunk = ChunkType::from_str("Ru1t");
        assert!(chunk.is_err());
    }

    #[test]
    pub fn test_chunk_type_string() {
        let chunk = ChunkType::from_str("RuSt").unwrap();
        assert_eq!(&chunk.to_string(), "RuSt");
    }

    #[test]
    pub fn test_chunk_type_trait_impls() {
        let chunk_type_1: ChunkType = TryFrom::try_from([82, 117, 83, 116]).unwrap();
        let chunk_type_2: ChunkType = FromStr::from_str("RuSt").unwrap();
        let _chunk_string = format!("{}", chunk_type_1);
        let _are_chunks_equal = chunk_type_1 == chunk_type_2;
    }
}
