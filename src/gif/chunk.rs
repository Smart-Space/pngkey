use std::fmt;

#[derive(Debug, Clone)]
pub enum Chunk {
    Header([u8; 6]),
    LogicalScreenDescriptor(LogicalScreenDescriptor),
    GlobalColorTable(Vec<u8>),
    Image(ImageChunk),
    Extension(ExtensionChunk),
    Trailer,
}

impl fmt::Display for Chunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Chunk::Header(data) => {
                write!(f, "Header: [{}]",
                       data.iter().map(|&b| format!("{:02x}", b)).collect::<Vec<_>>().join(", "))
            }
            Chunk::LogicalScreenDescriptor(data) => write!(f, "{}", data),
            Chunk::GlobalColorTable(data) => {
                write!(f, "GlobalColorTable: [{}]",
                       data.iter().map(|&b| format!("{:02x}", b)).collect::<Vec<_>>().join(", "))
            }
            Chunk::Image(data) => write!(f, "{}", data),
            Chunk::Extension(data) => write!(f, "{}", data),
            Chunk::Trailer => write!(f, "Trailer"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct LogicalScreenDescriptor {
    pub width: u16,
    pub height: u16,
    pub packed_fields: u8,
    pub background_color_index: u8,
    pub pixel_aspect_ratio: u8,
}

impl fmt::Display for LogicalScreenDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "LogicalScreenDescriptor {{")?;
        writeln!(f, "  width: {},", self.width)?;
        writeln!(f, "  height: {},", self.height)?;
        writeln!(f, "  packed_fields: {:#04x},", self.packed_fields)?;
        writeln!(f, "  background_color_index: {},", self.background_color_index)?;
        writeln!(f, "  pixel_aspect_ratio: {},", self.pixel_aspect_ratio)?;
        writeln!(f, "}}")?;
        Ok(())
    } 
}

#[derive(Debug, Clone)]
pub struct ImageChunk {
    pub descriptor: ImageDescriptor,
    pub local_color_table: Option<Vec<u8>>,
    pub image_data: Vec<u8>, // 包含LZW压缩数据的子块链
}

impl fmt::Display for ImageChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ImageChunk {{")?;
        writeln!(f, "  Descriptor: {}", self.descriptor)?;
        if let Some(_) = self.local_color_table {
            writeln!(f, "  <Local Color Table Data>")?;
        }
        writeln!(f, "  Image: <Image Data>")?;
        writeln!(f, "}}")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ImageDescriptor {
    pub left: u16,
    pub top: u16,
    pub width: u16,
    pub height: u16,
    pub packed_fields: u8,
}

impl fmt::Display for ImageDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ImageDescriptor {{")?;
        writeln!(f, "    left: {},", self.left)?;
        writeln!(f, "    top: {},", self.top)?;
        writeln!(f, "    width: {},", self.width)?;
        writeln!(f, "    height: {},", self.height)?;
        writeln!(f, "    packed_fields: {:#04x},", self.packed_fields)?;
        write!(f, "  }}")?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ExtensionChunk {
    pub extension_type: u8,
    pub data: Vec<u8>,
}

impl fmt::Display for ExtensionChunk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ExtensionChunk {{")?;
        writeln!(f, "  Extension type: {:#04x},", self.extension_type)?;
        if self.extension_type == 0xff {
            writeln!(f, "  Application Identifier: {}", self.data[1..9].iter().map(|&b| b as char).collect::<String>())?;
            writeln!(f, "  Application Authentication Code: {}", self.data[9..12].iter().map(|&b| b as char).collect::<String>())?;
            writeln!(f, "  Data: [{}]", String::from_utf8_lossy(&self.data[12..]))?; // 这里是data的原始数据，明文可能被截断
        } else {
            writeln!(f, "  ...Datas...")?;
        }
        writeln!(f, "}}")?;
        Ok(())
    }
}