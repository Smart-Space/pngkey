use std::convert::TryFrom;
use std::fmt;
use std::fs;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

use crate::chunk;
use crate::{Error, Result};
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;

/// PNG结构
#[derive(Debug)]
pub struct Png {
    header: [u8; 8],
    chunks: Vec<Chunk>,
}

impl Png {
    /// 固定开头
    pub const STANDARD_HEADER: [u8; 8] = [137, 80, 78, 71 ,13, 10, 26, 10];

    /// 从Chunks创建Png
    pub fn from_chunks(chunks: Vec<Chunk>) -> Self {
        Png {
            header: Png::STANDARD_HEADER,
            chunks,
        }
    }

    /// 添加chunk到png
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    /// 搜索特定chunk_type的Chunk并移除
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        let index = self
            .chunks.iter()
            .position(|chunk| format!("{}", chunk.chunk_type()) == chunk_type)
            .ok_or("Chunk not found")?;
        Ok(self.chunks.remove(index))
    }

    pub fn header(&self) -> &[u8; 8] {
        &self.header
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    /// 找到第一个符合条件的Chunk
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<&Chunk> {
        self.chunks
            .iter()
            .find(|chunk| format!("{}", chunk.chunk_type()) == chunk_type)
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        bytes.extend(&self.header);
        bytes.extend(
            self.chunks
               .iter()
               .flat_map(|chunk| chunk.as_bytes())
               .collect::<Vec<u8>>(),
        );
        bytes
    }
}

impl TryFrom<&[u8]> for Png {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Png> {
        if bytes.len() < 8 {
            return Err("PNG data is too short (<8)".into());
        }
        let header: [u8; 8] = bytes[..8].try_into().unwrap();
        if header != Png::STANDARD_HEADER {
            return Err("PNG header is invalid".into());
        }
        let mut chunks = Vec::new();
        let mut index = 8;
        while index < bytes.len() {
            let length_bytes: [u8; 4] = bytes[index..index+4].try_into().unwrap();
            let length = u32::from_be_bytes(length_bytes) as usize;
            let chunk_end = index + 4 + 4 + length + 4;
            if chunk_end > bytes.len() {
                return Err("PNG chunk is too long".into());
            }
            let chunk_bytes = &bytes[index..chunk_end];
            let chunk = Chunk::try_from(chunk_bytes)?;
            chunks.push(chunk);
            index = chunk_end;
        }
        Ok(Png { header, chunks })
    }
}

impl fmt::Display for Png {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "PNG:")?;
        for chunk in &self.chunks {
            writeln!(f, "{}", chunk)?;
        }
        Ok(())
    }
}