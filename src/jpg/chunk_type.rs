use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

/// JPG块类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    type_name: u8,
}

impl ChunkType {
    pub fn bytes(&self) -> u8 {
        self.type_name
    }
}

impl TryFrom<u8> for ChunkType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self> {
        Ok(ChunkType { type_name: value })
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.type_name)?;
        Ok(())
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 2 {
            return Err("Invalid chunk type format".into());
        }
        let type_name = u8::from_str_radix(s, 10)?;
        ChunkType::try_from(type_name)
    }
}
