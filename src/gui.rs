use std::path::PathBuf;

use crate::args;
use crate::commands;

slint::include_modules!();

pub fn run_gui() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    
    ui.on_encode(|file, chunk, content, key, output| {
        let v_key: Option<String> = if !key.is_empty() {
            Some(key.into())
        } else {
            None
        };
        let v_output: Option<PathBuf> = if !output.is_empty() {
            Some(PathBuf::from(&output))
        } else {
            None
        };
        let encodeargs = args::EncodeArgs {
            file_path: PathBuf::from(&file),
            chunk_type: chunk.into(),
            message: content.into(),
            output: v_output,
            password: v_key,
        };
        // println!("{:?}", encodeargs);
        if let Err(e) = commands::encode(encodeargs) {
            eprintln!("Error: {}", e);
        }
    });

    let ui_weak = ui.as_weak();
    ui.on_decode(move |file, chunk, key| {
        let v_key: Option<String> = if !key.is_empty() {
            Some(key.into())
        } else {
            None
        };
        let decodeargs = args::DecodeArgs {
            file_path: PathBuf::from(&file),
            chunk_type: chunk.into(),
            password: v_key,
        };
        // println!("{:?}", decodeargs);
        let ui = ui_weak.unwrap();
        ui.set_result_text("".into());
        match commands::decode(decodeargs) {
            Ok(content) => {
                ui.set_result_text(content.into());
            }
            Err(e) => {
                eprintln!("Error: {}", e);
            }
        }
    });

    ui.on_remove(|file, chunk| {
        let removeargs = args::RemoveArgs {
            file_path: PathBuf::from(&file),
            chunk_type: chunk.into(),
        };
        // println!("{:?}", removeargs);
        if let Err(e) = commands::remove(removeargs) {
            eprintln!("Error: {}", e);
        }
    });

    ui.run()?;
    Ok(())
}