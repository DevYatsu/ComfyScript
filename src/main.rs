mod comfy;
mod command;
mod exec;
pub mod minify;
pub mod parser;
mod reserved_keywords;

use minify::minify_input;

use crate::{
    command::{get_command, Command},
    exec::exec_script,
};

use std::{
    error::Error,
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
};

fn main() -> Result<(), Box<dyn Error>> {
    let command = get_command();

    match command {
        Command::RunFile(path) => {
            exec_script(&path)?;
        }
        Command::MinifyFile(path) => {
            minify_input(&path)?;
        }
        Command::NotFound => {
            return Err("Invalid command!".into());
        }
    }
    Ok(())
}

fn get_file_content(file_path: &Path) -> Result<String, Box<dyn Error>> {
    let file_metadata = fs::metadata(&file_path)?;

    if file_metadata.len() == 0 {
        println!("{} is empty", file_path.to_string_lossy());
        std::process::exit(1)
    }

    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}
