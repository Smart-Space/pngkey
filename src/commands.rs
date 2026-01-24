use std::fs;
use std::str::FromStr;

use crate::args::*;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;
use crate::key;


/// 判断能否使用
static VALID_CHUNK_TYPES: [&str; 23] = [
    "IHDR", "PLTE", "IDAT", "IEND", "acTL", "cHRM", "cICP", "gAMA", "iCCP", "mDCV", "cLLI",
    "sBIT", "sRGB", "bkGD", "hIST", "tRNS", "eXIf", "fcTL", "fdAT", "tIME", "zTXt", "iTXt", "tEXt",
];
fn is_valid_chunk_type(chunk_type_str: &str) -> bool {
    !VALID_CHUNK_TYPES.contains(&chunk_type_str)
}

pub fn encode(args: EncodeArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }

    // chunk type可用
    if !is_valid_chunk_type(&args.chunk_type) {
        return Err(format!("Invalid ChunkType, could not in {VALID_CHUNK_TYPES:?}.").into());
    }

    // 密钥与信息
    let password = args.password.unwrap_or_else(|| "".to_string());
    let message = args.message;
    let encrypted_message: String;
    if !password.is_empty() {
        encrypted_message = key::encrypt(&message, &password)?;
    } else {
        encrypted_message = message;
    }

    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&bytes[..])?;
    let new_chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        encrypted_message.as_bytes().to_vec(),
    );
    png.append_chunk(new_chunk);
    if let Some(output) = args.output {
        fs::write(output, png.as_bytes())?;
    } else {
        fs::write(args.file_path, png.as_bytes())?;
    }
    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }

    // chunk type可用
    if !is_valid_chunk_type(&args.chunk_type) {
        return Err(format!("Invalid ChunkType, could not in {VALID_CHUNK_TYPES:?}.").into());
    }

    let bytes = fs::read(args.file_path.clone())?;
    let png = Png::try_from(&bytes[..])?;
    let chunk = png
        .chunks()
        .iter()
        .find(|chunk| chunk.chunk_type().to_string() == args.chunk_type)
        .ok_or("Chunk not found")?;
    let message = std::str::from_utf8(chunk.data()).expect("Invalid UTF-8");
    let password = args.password.unwrap_or_else(|| "".to_string());
    let decrypted_message = key::decrypt(&message, &password)?;
    println!("{}", decrypted_message);
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&bytes[..])?;
    png.remove_chunk(&args.chunk_type)?;
    fs::write(args.file_path, png.as_bytes())?;
    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }

    let mut chunk_type: String = ("").to_string();
    if let Some(_chunk_type) = args.chunk_type {
        chunk_type = _chunk_type;
    }

    let bytes = fs::read(args.file_path.clone())?;
    let png = Png::try_from(&bytes[..])?;

    if chunk_type.is_empty() {
        for chunk in png.chunks() {
            println!("{}", chunk);
        }
    } else {
        let chunk = png
            .chunks()
            .iter()
            .find(|chunk| chunk.chunk_type().to_string() == chunk_type)
            .ok_or("Chunk not found")?;
        println!("{}", chunk);
    }
    Ok(())
}