use std::convert::TryFrom;
use std::fmt;
use std::io::{BufReader, Read};

use crc;

use crate::{Error, Result};
use crate::chunk_type::ChunkType;

/// PNG块
/// http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
#[derive(Debug, Clone)]
pub struct Chunk {
    length: u32,
    chunk_type: ChunkType,
    data: Vec<u8>,
    crc: u32,
}

impl Chunk {
    pub fn new(chunk_type: ChunkType, data: Vec<u8>) -> Chunk {
        let length = data.len() as u32;
        let crc_content = chunk_type
            .bytes()
            .iter()
            .chain(data.iter())
            .copied()
            .collect::<Vec<u8>>();
        let crc = crc::Crc::<u32>::new(&crc::CRC_32_ISO_HDLC).checksum(&crc_content);
        Chunk {
            length,
            chunk_type,
            data,
            crc,
        }
    }

    pub fn length(&self) -> u32 {
        self.length
    }

    pub fn chunk_type(&self) -> &ChunkType {
        &self.chunk_type
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn crc(&self) -> u32 {
        self.crc
    }

    pub fn data_as_string(&self) -> Result<String> {
        let string = String::from_utf8(self.data.clone())?;
        Ok(string)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.length.to_be_bytes());
        bytes.extend(&self.chunk_type.bytes());
        bytes.extend(&self.data);
        bytes.extend(&self.crc.to_be_bytes());
        bytes
    }
}

impl TryFrom<&[u8]> for Chunk {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 12 {
            return Err("Chunk data is too short (<32)".into())
        }
        let chunk_type_bytes: [u8; 4] = bytes[4..8].try_into().unwrap();
        let chunk_type = ChunkType::try_from(chunk_type_bytes)?;
        let data = bytes[8..bytes.len() - 4].to_vec();
        let crc_bytes: [u8; 4] = bytes[bytes.len() - 4..].try_into().unwrap();
        let crc = u32::from_be_bytes(crc_bytes);
        let chunk = Chunk::new(chunk_type, data);
        if chunk.crc != crc {
            return Err("CRC check failed".into())
        }
        Ok(chunk)
    }
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Chunk {{",)?;
        writeln!(f, "  Length: {}", self.length())?;
        writeln!(f, "  Type: {}", self.chunk_type())?;
        writeln!(f, "  Data: {}", self.data().len())?;
        writeln!(f, "  CRC: {}", self.crc())?;
        writeln!(f, "}}",)?;
        Ok(())
    }
}
