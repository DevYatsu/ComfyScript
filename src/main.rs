mod comfy;
mod command;
mod minify;
pub mod parser;
mod reserved_keywords;

use nom_locate::LocatedSpan;
use parser::parse_input;

use crate::command::{get_command, Command};

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
            let file_content = get_file_content(&path)?;
            let input = LocatedSpan::new_extra(file_content.as_str(), "");
            let e = parse_input(input, None);
            println!("{:?}", e);
        }
        Command::MinifyFile(path) => {
            let file_content = get_file_content(&path)?;
        }
        Command::NotFound => {
            println!("Invalid command!");
            std::process::exit(1)
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
