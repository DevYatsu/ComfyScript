mod comfy;
mod command;
mod errors;
mod execute_folder;
mod minify;
mod parser;
mod reserved_keywords;
mod script;

use crate::{
    command::{get_command, Command},
    minify::generate_minified_file,
    script::ComfyScript,
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
            if path.is_dir() {
                execute_folder::execute_folder(&path)?;
            } else {
                let content = get_file_content(&path)?;
                let script = ComfyScript::new(path.to_string_lossy(), content);

                match script.execute() {
                    Ok(_) => (),
                    Err((err, file)) => {
                        err.print(file).unwrap();
                    }
                };
            }
        }
        Command::MinifyFile(path) => {
            let content = get_file_content(&path)?;
            let script = ComfyScript::new(path.to_string_lossy(), content);

            let minified_script = script.minify()?;

            generate_minified_file(&path, minified_script.as_bytes())?;
        }
        Command::NotFound => {
            return Err("Command not found!".into());
        }
        Command::MissingFileName => {
            return Err("Missing a valid file name!".into());
        }
    }

    let elapsed_time = start_time.elapsed();
    println!("Execution time: {} microseconds", elapsed_time.as_micros(),);
    Ok(())
}

fn get_file_content(file_path: &Path) -> Result<String, Box<dyn Error>> {
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
