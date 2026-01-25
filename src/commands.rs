use std::fs;

use crate::args::*;
use crate::png::is_png;
use crate::png::command as pngcommand;
use crate::Result;

pub fn encode(args: EncodeArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;

    if is_png(&bytes) {
        pngcommand::encode(args, &bytes)?;
    }

    Ok(())
}

pub fn decode(args: DecodeArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;

    if is_png(&bytes) {
        pngcommand::decode(args, &bytes)?;
    }
    
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;
    
    pngcommand::remove(args, &bytes)?;

    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;

    pngcommand::print(args, &bytes)?;
    
    Ok(())
}