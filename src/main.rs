use clap::Parser;

mod args;
mod commands;
mod png;
mod jpg;
mod key;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Cli::parse();
    println!("{:?}", args);
    let res = match args.subcommand {
        args::PngKeyArgs::Encode(encode_args) => commands::encode(encode_args),
        args::PngKeyArgs::Decode(decode_args) => commands::decode(decode_args),
        args::PngKeyArgs::Remove(remove_args) => commands::remove(remove_args),
        args::PngKeyArgs::Print(print_args) => commands::print(print_args),
    };
    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
