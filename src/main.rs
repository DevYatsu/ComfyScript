mod comfy;
mod command;
mod errors;
mod exec;
mod minify;
mod parser;
mod reserved_keywords;
mod test_files;

use errors::ComfyScriptError;
use miette::Context;
use minify::minify_input;
use test_files::parse_all_files;

use crate::{
    command::{get_command, Command},
    exec::exec_script,
};

use std::{
    error::Error,
    fs::{self, File},
    io::{BufReader, Read},
    path::Path,
    time::Instant,
};

fn main() -> Result<(), Box<dyn Error>> {
    let start_time = Instant::now();
    let command = get_command();

    match command {
        Command::RunFile(path) => {
            exec_script(&path).wrap_err("Syntax Error")?;
        }
        Command::MinifyFile(path) => {
            minify_input(&path)?;
        }
        Command::NotFound => {
            return Err("Command not found!".into());
        }
        Command::MissingFileName => {
            return Err("Missing a valid file name!".into());
        }
        Command::TestFiles => parse_all_files()?,
    }

    let elapsed_time = start_time.elapsed();
    println!(
        "Execution time: {} seconds and {} milliseconds",
        elapsed_time.as_secs(),
        elapsed_time.subsec_millis()
    );
    Ok(())
}

fn get_file_content(file_path: &Path) -> Result<String, ComfyScriptError> {
    let file_metadata = fs::metadata(&file_path)?;

    if file_metadata.len() == 0 {
        return Ok(String::new());
    }

    let file = File::open(&file_path)?;
    let mut reader = BufReader::new(file);
    let mut content = String::new();
    reader.read_to_string(&mut content)?;

    Ok(content)
}
