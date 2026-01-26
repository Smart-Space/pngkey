use std::fs;
use std::str::FromStr;

use crate::args::*;
use super::chunk::Chunk;
use super::chunk_type::ChunkType;
use super::Jpg;
use crate::Result;
use crate::key;


fn is_valid_chunk_type(chunk_type_str: &str) -> bool {
    let chunk_type_u8 = u8::from_str(chunk_type_str).unwrap_or(0);
    0x01 <= chunk_type_u8 && chunk_type_u8 <= 0xbf
}

pub fn encode(args: EncodeArgs, bytes: &Vec<u8>) -> Result<()> {
    if !is_valid_chunk_type(&args.chunk_type) {
        return Err("Invalid ChunkType, should 1<= chunk-type <= 191.".into());
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

    let mut jpg = Jpg::try_from(bytes.as_slice())?;

    if let Some(index) = jpg.chunk_by_type(&args.chunk_type) {
        jpg.modify_chunk(index, encrypted_message.as_bytes().to_vec());
    } else {
        let new_chunk = Chunk::new(
            ChunkType::from_str(&args.chunk_type)?,
            encrypted_message.as_bytes().to_vec(),
        );
        jpg.append_chunk(new_chunk);
    }

    if let Some(output_path) = args.output {
        fs::write(output_path, jpg.as_bytes())?;
    } else {
        fs::write(args.file_path, jpg.as_bytes())?;
    }

    Ok(())
}

pub fn decode(args: DecodeArgs, bytes: &Vec<u8>) -> Result<()> {
    if !is_valid_chunk_type(&args.chunk_type) {
        return Err("Invalid ChunkType, should 1<= chunk-type <= 191.".into());
    }

    let jpg = Jpg::try_from(bytes.as_slice())?;
    let chunk = jpg
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

pub fn remove(args: RemoveArgs, bytes: &Vec<u8>) -> Result<()> {
    let mut jpg = Jpg::try_from(bytes.as_slice())?;
    jpg.remove_chunk(&args.chunk_type)?;
    fs::write(args.file_path, jpg.as_bytes())?;
    Ok(())
}

pub fn print(args: PrintArgs, bytes: &Vec<u8>) -> Result<()> {
    let mut chunk_type: String = "".to_string();
    if let Some(ct) = args.chunk_type {
        chunk_type = ct;
    }

    let jpg = Jpg::try_from(bytes.as_slice())?;

    if chunk_type.is_empty() {
        for chunk in jpg.chunks() {
            println!("{}", chunk);
        }
    } else {
        let chunk = jpg
            .chunks()
            .iter()
            .find(|chunk| chunk.chunk_type().to_string() == chunk_type)
            .ok_or("Chunk not found")?;
        println!("{}", chunk);
    }

    Ok(())
}
