use std::io::Write;
use std::{convert::TryFrom, io::Read};
use std::{fmt, vec};

mod chunk;

use crate::{Error, Result};
use chunk::*;

/// GIF结构
#[derive(Debug)]
pub struct Gif {
    chunks: Vec<Chunk>,
}

impl Gif {
    pub fn append_chunk(&mut self, chunk: ExtensionChunk) {
        self.chunks.push(Chunk::Extension(chunk));
    }

    pub fn remove_chunk(&mut self, chunk_type: &str) -> Result<Chunk> {
        if let Some(index) = self.chunk_by_type(chunk_type) {
            return Ok(self.chunks.remove(index));
        } else {
            return Err(format!("GIF does not contain chunk type {}", chunk_type).into());
        }
    }

    pub fn modify_chunk(&mut self, index: usize, data: Vec<u8>) {
        if let Chunk::Extension(chunk) = &mut self.chunks[index] {
            let identifier = chunk.data[0..8].to_vec();
            let auth_code = chunk.data[8..11].to_vec();
            let mut new_data = Vec::new();
            new_data.extend_from_slice(&identifier);
            new_data.extend_from_slice(&auth_code);
            new_data.extend_from_slice(&data);
            chunk.data = new_data;
        }
    }

    pub fn chunks(&self) -> &[Chunk] {
        &self.chunks
    }

    pub fn chunk_by_type(&self, chunk_type: &str) -> Option<usize> {
        let index = self
            .chunks.iter()
            .position(|c| matches!(c, Chunk::Extension(e) if e.extension_type == 0xFF && e.data[0..8].to_vec() == " pngkey ".as_bytes().to_vec() && e.data[8..11].to_vec() == chunk_type.as_bytes().to_vec()));
        index
    }

    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        let mut bytes: Vec<u8> = Vec::new();

        for chunk in &self.chunks {
            match chunk {
                Chunk::Header(header) => {
                    bytes.write_all(header)?;
                }
                Chunk::LogicalScreenDescriptor(lsd) => {
                    bytes.write_all(&lsd.width.to_be_bytes())?;
                    bytes.write_all(&lsd.height.to_be_bytes())?;
                    bytes.write_all(&[lsd.packed_fields])?;
                    bytes.write_all(&[lsd.background_color_index])?;
                    bytes.write_all(&[lsd.pixel_aspect_ratio])?;
                }
                Chunk::GlobalColorTable(gct) => {
                    bytes.write_all(gct)?;
                }
                Chunk::Image(image) => {
                    bytes.write_all(&[0x2c])?;
                    bytes.write_all(&image.descriptor.left.to_be_bytes())?;
                    bytes.write_all(&image.descriptor.top.to_be_bytes())?;
                    bytes.write_all(&image.descriptor.width.to_be_bytes())?;
                    bytes.write_all(&image.descriptor.height.to_be_bytes())?;
                    bytes.write_all(&[image.descriptor.packed_fields])?;
                    
                    if let Some(lct) = &image.local_color_table {
                        bytes.write_all(lct)?;
                    }

                    bytes.write_all(&image.image_data)?;
                }
                Chunk::Extension(extension) => {
                    bytes.write_all(&[0x21])?;
                    bytes.write_all(&[extension.extension_type])?;
                    bytes.write_all(&extension.data)?;
                }
                Chunk::Trailer => {
                    bytes.write_all(&[0x3b])?;
                }
            }
        }

        Ok(bytes)
    }

    /// 添加自定义Application Extension（用于存储UTF-8数据）
    pub fn add_application_extension(&mut self, identifier: &[u8; 8], auth_code: &[u8; 3], data: &[u8]) -> Result<()> {
        // 构建Application Extension数据
        let mut ext_data = Vec::new();
        
        // Application Extension头部（11字节）
        ext_data.extend_from_slice(identifier);
        ext_data.extend_from_slice(auth_code);
        
        // 添加数据子块（每个子块最多254字节数据 + 1字节长度）
        let mut remaining_data = data;
        while !remaining_data.is_empty() {
            let chunk_size = std::cmp::min(remaining_data.len(), 254);
            ext_data.push(chunk_size as u8);
            ext_data.extend_from_slice(&remaining_data[..chunk_size]);
            remaining_data = &remaining_data[chunk_size..];
        }
        
        // 结束标记
        ext_data.push(0x00);
        
        // 创建扩展块（类型0xFF = Application Extension）
        let extension = ExtensionChunk {
            extension_type: 0xFF,
            data: ext_data,
        };
        
        // 插入到Trailer之前
        if let Some(trailer_pos) = self.chunks.iter().rposition(|c| matches!(c, Chunk::Trailer)) {
            self.chunks.insert(trailer_pos, Chunk::Extension(extension));
        } else {
            self.chunks.push(Chunk::Extension(extension));
        }
        
        Ok(())
    }
    
    /// 提取所有Application Extension中的数据
    pub fn extract_application_extensions(&self, code: &str) -> Vec<(String, String, Vec<u8>)> {
        let mut results = Vec::new();
        
        for chunk in &self.chunks {
            if let Chunk::Extension(ext) = chunk {
                if ext.extension_type == 0xFF && ext.data.len() >= 11 {
                    let identifier = String::from_utf8_lossy(&ext.data[0..8]).to_string();
                    let auth_code = String::from_utf8_lossy(&ext.data[8..11]).to_string();
                    
                    // 解析子块数据
                    let mut data = Vec::new();
                    let mut pos = 11;
                    while pos < ext.data.len() {
                        let block_size = ext.data[pos] as usize;
                        if block_size == 0 {
                            break; // 结束标记
                        }
                        pos += 1;
                        if pos + block_size > ext.data.len() {
                            break; // 数据损坏
                        }
                        data.extend_from_slice(&ext.data[pos..pos + block_size]);
                        pos += block_size;
                    }
                    
                    results.push((identifier, auth_code, data));
                }
            }
        }
        
        results
    }

    /// 内部辅助方法
    
    fn read_logical_screen_descriptor<R: Read>(reader: &mut R) -> Result<(LogicalScreenDescriptor)> {
        let mut buf = [0u8; 7];
        reader.read_exact(&mut buf)?;
        
        Ok(LogicalScreenDescriptor {
            width: u16::from_le_bytes([buf[0], buf[1]]),
            height: u16::from_le_bytes([buf[2], buf[3]]),
            packed_fields: buf[4],
            background_color_index: buf[5],
            pixel_aspect_ratio: buf[6],
        })
    }
    
    fn read_image_chunk<R: Read>(reader: &mut R) -> Result<(ImageChunk)> {
        let mut buf = [0u8; 9];
        reader.read_exact(&mut buf)?;
        
        let descriptor = ImageDescriptor {
            left: u16::from_le_bytes([buf[0], buf[1]]),
            top: u16::from_le_bytes([buf[2], buf[3]]),
            width: u16::from_le_bytes([buf[4], buf[5]]),
            height: u16::from_le_bytes([buf[6], buf[7]]),
            packed_fields: buf[8],
        };
        
        // 检查局部调色板
        let local_color_table = if (descriptor.packed_fields & 0x80) != 0 {
            let size = 2u16.pow(((descriptor.packed_fields & 0x07) + 1) as u32) as usize;
            let mut lct = vec![0u8; size * 3];
            reader.read_exact(&mut lct)?;
            Some(lct)
        } else {
            None
        };
        
        // 读取图像数据（LZW压缩数据的子块链）
        let image_data = Self::read_sub_blocks(reader)?;
        
        Ok(ImageChunk {
            descriptor,
            local_color_table,
            image_data,
        })
    }
    
    fn read_extension_chunk<R: Read>(reader: &mut R) -> Result<(ExtensionChunk)> {
        let mut ext_type = [0u8; 1];
        reader.read_exact(&mut ext_type)?;
        
        let data = Self::read_sub_blocks(reader)?;
        
        Ok(ExtensionChunk {
            extension_type: ext_type[0],
            data,
        })
    }
    
    fn read_sub_blocks<R: Read>(reader: &mut R) -> Result<Vec<u8>> {
        let mut data = Vec::new();
        loop {
            let mut size_byte = [0u8; 1];
            reader.read_exact(&mut size_byte)?;
            
            let size = size_byte[0] as usize;
            if size == 0 {
                break; // 子块链结束
            }
            
            let mut block_data = vec![0u8; size];
            reader.read_exact(&mut block_data)?;
            data.extend_from_slice(&block_data);
        }
        Ok(data)
    }
}

pub fn is_gif(bytes: &[u8]) -> bool {
    &bytes[0..6] == b"GIF89a"
}

impl TryFrom<&[u8]> for Gif {
    type Error = Error;

    fn try_from(bytes: &[u8]) -> Result<Gif> {
        let mut cursor = std::io::Cursor::new(bytes);
        let mut chunks = Vec::new();
        
        let mut header = [0u8; 6];
        cursor.read_exact(&mut header)?;
        chunks.push(Chunk::Header(header));
        // if !(&header[..3] == b"GIF") {
        //     return Err("Invalid GIF header".into());
        // }

        let lsd = Self::read_logical_screen_descriptor(&mut cursor)?;
        let has_gct = (lsd.packed_fields & 0x80) != 0;
        let gct_size_factor = (lsd.packed_fields & 0x70);
        chunks.push(Chunk::LogicalScreenDescriptor(lsd));

        let global_color_table_size = if has_gct {
            let size = 2u16.pow((gct_size_factor+1) as u32) as usize;
            let mut gct = vec![0u8; size * 3]; // RGB / 1 byte
            cursor.read_exact(&mut gct)?;
            chunks.push(Chunk::GlobalColorTable(gct));
            Some(size)
        } else {
            None
        };

        // 读取所有数据直到Trailer
        loop {
            let mut block_type = [0u8; 1];
            cursor.read_exact(&mut block_type)?;

            match block_type[0] {
                0x2c => {
                    // ','分割图像
                    let image = Self::read_image_chunk(&mut cursor)?;
                    chunks.push(Chunk::Image(image));
                }
                0x21 => {
                    // '!'拓展块
                    let extension = Self::read_extension_chunk(&mut cursor)?;
                    chunks.push(Chunk::Extension(extension));
                }
                0x3b => {
                    // 结尾
                    chunks.push(Chunk::Trailer);
                    break;
                }
                _ => {
                    return Err("Invalid GIF block type".into());
                }
            }
        }
        Ok(Gif { chunks })
    }
}

impl fmt::Display for Gif {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GIF:")?;
        for chunk in &self.chunks {
            writeln!(f, "{}", chunk)?;
        }
        Ok(())
    }
}
