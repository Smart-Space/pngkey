use std::fs;
use std::str::FromStr;

use crate::args::*;
use crate::chunk::Chunk;
use crate::chunk_type::ChunkType;
use crate::png::Png;
use crate::Result;

pub fn encode(args: EncodeArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;
    let mut png = Png::try_from(&bytes[..])?;
    let new_chunk = Chunk::new(
        ChunkType::from_str(&args.chunk_type)?,
        args.message.as_bytes().to_vec(),
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
    let bytes = fs::read(args.file_path.clone())?;
    let png = Png::try_from(&bytes[..])?;
    let chunk = png
        .chunks()
        .iter()
        .find(|chunk| chunk.chunk_type().to_string() == args.chunk_type)
        .ok_or("Chunk not found")?;
    let message: String = chunk.data().iter().map(|x| *x as char).collect();
    println!("{}", message);
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
    let bytes = fs::read(args.file_path.clone())?;
    let png = Png::try_from(&bytes[..])?;
    for chunk in png.chunks() {
        println!("{}", chunk);
    }
    Ok(())
}