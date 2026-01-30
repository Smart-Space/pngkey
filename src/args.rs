use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about,
    long_about = None
)]
pub struct Cli {
    #[clap(subcommand)]
    pub subcommand: Option<PngKeyArgs>,
}

#[derive(Debug, Subcommand)]
pub enum PngKeyArgs {
    Encode(EncodeArgs),
    Decode(DecodeArgs),
    Remove(RemoveArgs),
    Print(PrintArgs),
}

#[derive(Debug, Args)]
pub struct EncodeArgs {
    /// The file path to the Image file to be encoded.
    pub file_path: PathBuf,
    /// The chunk type to be used for the message.
    pub chunk_type: String,
    /// The message to be encoded.
    pub message: String,
    /// The output file path. If not specified, the original file will be overwritten.
    #[clap(short, long)]
    pub output: Option<PathBuf>,
    /// The password to be used for encryption. If not specified, the message will be stored in plain text.
    #[clap(short, long)]
    pub password: Option<String>,
}

#[derive(Debug, Args)]
pub struct DecodeArgs {
    /// The file path to the Image file to be decoded.
    pub file_path: PathBuf,
    /// The chunk type to be used for the message.
    pub chunk_type: String,
    /// The password to be used for decryption. If not specified, will show the message in plain text.
    #[clap(short, long)]
    pub password: Option<String>,
}

#[derive(Debug, Args)]
pub struct RemoveArgs {
    /// The file path to the Image file to be removed.
    pub file_path: PathBuf,
    /// The chunk type to be used for the message.
    pub chunk_type: String,
}

#[derive(Debug, Args)]
pub struct PrintArgs {
    /// The file path to the Image file to be printed.
    pub file_path: PathBuf,
    /// The chunk type to be used for the message.
    pub chunk_type: Option<String>,
    /// Show all chunks in the GIF file.
    #[clap(short, long)]
    pub all: bool,
}