use std::convert::TryFrom;
use std::fmt;

use crate::{Error, Result};


/// JPG块
#[derive(Debug, Clone)]
pub struct Chunk {
    head: u8,
    chunk_type: u8,
    length: u16,
    data: Vec<u8>,
}

static AVOID_LENGTH_TYPE: [u8; 3] = [0xd8, 0xd9, 0xda];

impl Chunk {
    pub fn new(chunk_type: u8, data: Vec<u8>) -> Chunk {
        let length: u16;
        if AVOID_LENGTH_TYPE.contains(&chunk_type) {
            if chunk_type == 0xda {
                // DA的长度放在数据里，这里只是显示头长度
                length = u16::from_be_bytes([data[0], data[1]]);
            } else {
                length = 0;
            }
        } else {
            length = u16::try_from(data.len()).unwrap() + 2;
        }
        Chunk {
            head: 0xff,
            chunk_type,
            length,
            data,
        }
    }

    pub fn length(&self) -> u16 {
        self.length
    }

    pub fn chunk_type(&self) -> &u8 {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn set_data(&mut self, data: Vec<u8>) {
        self.length = u16::try_from(data.len()).unwrap() + 2;
        self.data = data;
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.head.to_be_bytes());
        bytes.extend(&self.chunk_type.to_be_bytes());
        if !AVOID_LENGTH_TYPE.contains(&self.chunk_type) {
            bytes.extend(&self.length.to_be_bytes());
        }
        bytes.extend(&self.data);
        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(value: &[u8]) -> Result<Self> {
        if value.len() < 2 {
            return Err("Chunk data is too short (<2)".into());
        }
        let head = value[0];
        if head != 0xff {
            return Err("Invalid chunk head".into());
        }
        let chunk_type = value[1];
        let data = value[4..].to_vec();
        let chunk = Chunk::new(chunk_type, data);
        Ok(chunk)
    }
}

static AVOID_TYPE: [u8; 2] = [0xc4, 0xda];
impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let data: String;
        let chunk_type_name = self.chunk_type;
        if AVOID_TYPE.contains(&chunk_type_name) {
            data = format!("<{:02X} DATA>", chunk_type_name);
        } else {
            data = String::from_utf8_lossy(self.data()).to_string();
        }
        writeln!(f, "Chunk {{")?;
        writeln!(f, "  Lenghth: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {}", data)?;
        writeln!(f, "}}")?;
        Ok(())
    } 
}
