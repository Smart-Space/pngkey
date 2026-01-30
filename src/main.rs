use clap::Parser;

mod args;
mod commands;
mod png;
mod jpg;
mod gif;
mod key;

#[cfg(feature = "gui")]
mod gui;


pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

fn main() -> Result<()> {
    let args = args::Cli::parse();
    // println!("{:?}", args);
    let res = match args.subcommand {
        Some(args::PngKeyArgs::Encode(encode_args)) => commands::encode(encode_args),
        Some(args::PngKeyArgs::Decode(decode_args)) => {
            let _ = commands::decode(decode_args)?;
            Ok(())
        },
        Some(args::PngKeyArgs::Remove(remove_args)) => commands::remove(remove_args),
        Some(args::PngKeyArgs::Print(print_args)) => commands::print(print_args),
        None => {
            #[cfg(feature = "gui")]
            {
                let _ = gui::run_gui();
                Ok(())
            }

            #[cfg(not(feature = "gui"))]
            {
                eprintln!("Error: No subcommand provided. use --help for more information.");
                Ok(())
            }
        },
    };
    if let Err(e) = res {
        eprintln!("Error: {}", e);
    }

    Ok(())
}
