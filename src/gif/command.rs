use std::fs;

use crate::args::*;
use super::Gif;
use crate::Result;
use crate::key;

fn is_valid_chunk_type(chunk_type: &str) -> bool {
    let bytes = chunk_type.as_bytes();
    bytes.len() == 3
}

static IDENTIFIER: [u8; 8] = [b' ', b'p', b'n', b'g', b'k', b'e', b'y', b' '];

pub fn encode(args: EncodeArgs, bytes: &Vec<u8>) -> Result<()> {
    // chunk type可用
    if !is_valid_chunk_type(&args.chunk_type) {
        return Err(format!("Invalid ChunkType, should be 3 bytes long.").into());
    }

    let password = args.password.unwrap_or_else(|| "".to_string());
    let message = args.message;
    let encrypted_message: String;
    if !password.is_empty() {
        encrypted_message = key::encrypt(&message, &password)?;
    } else {
        encrypted_message = message;
    }

    let mut gif = Gif::try_from(bytes.as_slice())?;

    let chunk_type: [u8; 3] = args.chunk_type.as_bytes().try_into().unwrap();
    if let Some(index) = gif.chunk_by_type(&args.chunk_type) {
        gif.modify_chunk(index, encrypted_message.as_bytes().to_vec());
    } else {
        gif.add_application_extension(&IDENTIFIER, &chunk_type, encrypted_message.as_bytes())?;
    }

    if let Some(output) = args.output {
        fs::write(output, gif.as_bytes()?)?;
    } else {
        fs::write(args.file_path, gif.as_bytes()?)?;
    }

    Ok(())
}

pub fn decode(args: DecodeArgs, bytes: &Vec<u8>) -> Result<()> {
    // chunk type可用
    if !is_valid_chunk_type(&args.chunk_type) {
        return Err(format!("Invalid ChunkType, should be 3 bytes long.").into());
    }

    let gif = Gif::try_from(bytes.as_slice())?;
    let message_bytes = gif.extract_application_extensions(&args.chunk_type);
    if message_bytes.is_none() {
        return Err("Chunk not found".into());
    }
    let message = String::from_utf8(message_bytes.unwrap())?;
    let password = args.password.unwrap_or_else(|| "".to_string());
    let decrypted_message = key::decrypt(&message, &password)?;
    println!("{}", decrypted_message);
    Ok(())
}

pub fn remove(args: RemoveArgs, bytes: &Vec<u8>) -> Result<()> {
    let mut gif = Gif::try_from(bytes.as_slice())?;
    gif.remove_chunk(&args.chunk_type)?;
    fs::write(args.file_path, gif.as_bytes()?)?;
    Ok(())
}

pub fn print(args: PrintArgs, bytes: &Vec<u8>) -> Result<()> {
    let mut chunk_type: String = "".to_string();
    if let Some(_chunk_type) = args.chunk_type {
        chunk_type = _chunk_type;
    }

    let gif = Gif::try_from(bytes.as_slice())?;

    if chunk_type.is_empty() {
        for chunk in &gif.chunks {
            println!("{}", chunk);
        }
    } else {
        if let Some(index) = gif.chunk_by_type(&chunk_type) {
            println!("{}", gif.chunks[index]);
        } else {
            return Err("Chunk not found".into());
        }
    }
    Ok(())
}
