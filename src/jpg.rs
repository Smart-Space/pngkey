use std::convert::TryFrom;
use std::fmt;

mod chunk;
pub mod command;

use crate::{Error, Result};
use chunk::Chunk;


/// JPG结构
pub struct Jpg {
    header: [u8; 2],
    chunks: Vec<Chunk>,
}

impl Jpg {
    /// 固定开头
    pub const STANDARD_HEADER: [u8; 3] = [0xff, 0xd8, 0xff];

    /// 添加chunk到jpg
    pub fn append_chunk(&mut self, chunk: Chunk) {
        self.chunks.push(chunk);
    }

    /// 搜多特定chunk_type的chunk并移除
    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        if let Some(index) = self.chunk_by_type(chunk_type) {
            return Ok(self.chunks.remove(index));
        } else {
            return Err(format!("JPG dose not contain chunk type {}", chunk_type).into());
        }
    }

    /// 修改特定位置的chunk
    pub fn modify_chunk(&mut self, index: usize, data: Vec<u8>) {
        self.chunks[index].set_data(data);
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    /// 找到第一个Chunk
    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<usize> {
        let chunk_type = u8::from_str_radix(chunk_type, 10).unwrap();
        let index = self
            .chunks.iter()
            .position(|chunk| chunk.chunk_type() == &chunk_type);
        index
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

pub fn is_jpg(bytes: &[u8]) -> bool {
    if bytes.len() < 3 {
        return false;
    }
    let header: [u8; 3] = [bytes[0], bytes[1], bytes[2]];
    header == Jpg::STANDARD_HEADER
}

fn find_next_marker(bytes: &[u8], start: usize) -> Option<usize> {
    let mut i = start;
    while i + 1 < bytes.len() {
        if bytes[i] == 0xFF && bytes[i+1] != 0x00 {
            return Some(i);
        }
        i += 1;
    }
    None
}

impl TryFrom<&[u8]> for Jpg {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Jpg> {
        let header: [u8; 2] = [bytes[0], bytes[1]];
        let mut chunks = Vec::new();
        let mut index = 2;
        while index < bytes.len() {
            let marker_type = bytes[index + 1];
            index += 2;

            match marker_type {
                0xD8 => { // SOI
                    chunks.push(Chunk::new(0xD8, (&[]).to_vec()));
                }
                0xD9 => { // EOI
                    chunks.push(Chunk::new(0xD9, (&[]).to_vec()));
                    // JPG标准结束
                }
                0xDA => { // SOS
                    // 找到下一个 marker
                    if let Some(next_marker_pos) = find_next_marker(bytes, index) {
                        let chunk_bytes = &bytes[index..next_marker_pos];
                        chunks.push(Chunk::new(0xDA, chunk_bytes.to_vec()));
                        index = next_marker_pos;
                    } else {
                        return Err("No marker found after SOS chunk".into());
                    }
                }
                _ => {
                    let length_bytes: [u8; 2] = bytes[index..index+2].try_into().unwrap();
                    let length = u16::from_be_bytes(length_bytes) as usize;
                    let chunk_end = index + length;
                    let chunk_bytes = &bytes[index+2..chunk_end];
                    chunks.push(Chunk::new(marker_type, chunk_bytes.to_vec()));
                    index = chunk_end;
                }
            }

        }
        Ok(Jpg { header, chunks })
    }
}

impl fmt::Display for Jpg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "JGP:")?;
        for chunk in &self.chunks {
            writeln!(f, "{}", chunk)?;
        }
        Ok(())
    }
}
