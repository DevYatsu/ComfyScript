use std::{error::Error, fs};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::exec::exec_script;

pub fn parse_all_files() -> Result<(), Box<dyn Error>> {
    let folder_path = "tests/";

    let files = fs::read_dir(folder_path)
        .map_err(|e| {
            eprintln!("Error reading directory: {}", e);
            Box::new(e) as Box<dyn Error>
        })?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect::<Vec<_>>();

    files.par_iter().for_each(|file_path| {
        if let Err(e) = exec_script(file_path) {
            eprintln!(
                "\x1b[31mError executing script \x1b[33m{}\x1b[31m: {}\x1b[0m",
                file_path.display(),
                e
            )
        } else {
            println!(
                "\x1b[33m{}\x1b[32m successfully executed!\x1b[0m",
                file_path.display()
            )
        }
    });

    Ok(())
}
