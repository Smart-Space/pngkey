use std::convert::TryFrom;
use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

/// PNG块类型
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChunkType {
    first_byte: u8,
    second_byte: u8,
    third_byte: u8,
    fourth_byte: u8,
}

impl ChunkType {
    /// 原始数据
    pub fn bytes(&self) -> [u8; 4] {
        [
            self.first_byte,
            self.second_byte,
            self.third_byte,
            self.fourth_byte
        ]
    }

    // /// 辅助位
    // pub fn is_critical(&self) -> bool {
    //     self.first_byte & 0b0010_0000 == 0
    // }

    // /// 私有位
    // pub fn is_public(&self) -> bool {
    //     self.second_byte & 0b0010_0000 == 0
    // }

    // /// 保留位
    // pub fn is_reserved_bit_valid(&self) -> bool {
    //     self.third_byte & 0b0010_0000 == 0
    // }

    // /// 安全复制位
    // pub fn is_safe_to_copy(&self) -> bool {
    //     self.fourth_byte & 0b0010_0000 == 1
    // }

    // /// 如果保留字节有效并且所有四个字节均由字符A-Z或a-z表示，则返回true。
    // /// 此块类型应该始终有效
    // pub fn is_valid(&self) -> bool {
    //     self.is_reserved_bit_valid()
    // }
}

/// 判断是否为字母
fn is_all_alphabetic(bytes: [u8; 4]) -> bool {
    bytes.iter().all(|byte| byte.is_ascii_alphabetic())
}

impl TryFrom<[u8; 4]> for ChunkType {
    type Error = Error;

    fn try_from(bytes: [u8; 4]) -> Result<Self> {
        if !is_all_alphabetic(bytes) {
            return Err("ChunkType must be alphabetic".into());
        }
        let chunk_type = ChunkType {
            first_byte: bytes[0],
            second_byte: bytes[1],
            third_byte: bytes[2],
            fourth_byte: bytes[3],
        };
        Ok(chunk_type)
    }
}

impl fmt::Display for ChunkType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let bytes = self.bytes();
        for byte in bytes.iter() {
            write!(f, "{}", *byte as char)?;
        }
        Ok(())
    }
}

impl FromStr for ChunkType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.len() != 4 {
            return Err("ChunkType string must be 4 chars long".into());
        }
        let mut bytes = [0u8; 4];
        for (i, byte) in s.bytes().enumerate() {
            bytes[i] = byte;
        }
        ChunkType::try_from(bytes)
    }
}
