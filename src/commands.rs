use std::fs;

use crate::args::*;
use crate::png::is_png;
use crate::png::command as pngcommand;
use crate::jpg::is_jpg;
use crate::jpg::command as jpgcommand;
use crate::gif::is_gif;
use crate::gif::command as gifcommand;
use crate::Result;

pub fn encode(args: EncodeArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;

    if is_png(&bytes) {
        pngcommand::encode(args, &bytes)?;
    } else if is_jpg(&bytes) {
        jpgcommand::encode(args, &bytes)?;
    } else if is_gif(&bytes) {
        gifcommand::encode(args, &bytes)?;
    } else {
        return Err("No Supported Format".into());
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
    } else if is_jpg(&bytes) {
        jpgcommand::decode(args, &bytes)?;
    } else if is_gif(&bytes) {
        gifcommand::decode(args, &bytes)?;
    } else {
        return Err("No Supported Format".into());
    }
    
    Ok(())
}

pub fn remove(args: RemoveArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;
    
    if is_png(&bytes) {
        pngcommand::remove(args, &bytes)?;
    } else if is_jpg(&bytes) {
        jpgcommand::remove(args, &bytes)?;
    } else if is_gif(&bytes) {
        gifcommand::remove(args, &bytes)?;
    } else {
        return Err("No Supported Format".into());
    }

    Ok(())
}

pub fn print(args: PrintArgs) -> Result<()> {
    if !args.file_path.exists() {
        return Err("File does not exist".into());
    }
    let bytes = fs::read(args.file_path.clone())?;

    if is_png(&bytes) {
        pngcommand::print(args, &bytes)?;
    } else if is_jpg(&bytes) {
        jpgcommand::print(args, &bytes)?;
    } else if is_gif(&bytes) {
        gifcommand::print(args, &bytes)?;
    } else {
        return Err("No Supported Format".into());
    }
    
    Ok(())
}