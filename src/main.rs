mod command;
mod errors;
mod parser;
mod reserved_keywords;
mod comfy;

use parser::parse_input;

use crate::command::{get_command, Command};

use std::{
    error::Error,
    fs::{self, File},
    io::{BufReader, Read},
};

fn main() -> Result<(), Box<dyn Error>> {
    let file_path = match get_command() {
        Command::RunFile(path) => path,
        Command::NotFound => {
            println!("Invalid command!");
            std::process::exit(1)
        } // temporary solution until more commands are add
    };

    let file_metadata = fs::metadata(&file_path)?;

    if file_metadata.len() == 0 {
        println!("{} is empty", file_path.to_string_lossy());
        std::process::exit(1)
    }

    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    match parse_input(&content) {
        Err(e) => match e {
            nom::Err::Error(e) => {
                println!("{:?}", e);
            }
            nom::Err::Failure(e) => println!("{} at '{}'", e.errors[0].0, e.errors[1].0),
            _ => unreachable!(),
        },
        Ok(_) => println!("working"),
    };

    Ok(())
}
