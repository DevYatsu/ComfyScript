mod comfy;
mod command;
pub mod parser;
mod reserved_keywords;

use nom::{Finish, error::convert_error};
use nom_locate::LocatedSpan;
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

    let input = LocatedSpan::new_extra(content.as_str(), "");
    let e = parse_input(input, None);
    println!("{:?}", e);
    Ok(())
}
