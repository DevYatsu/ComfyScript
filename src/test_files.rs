use std::{error::Error, fs, time::Instant};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

pub fn parse_all_files() -> Result<(), Box<dyn Error>> {
    use crate::exec::exec_script;

    let folder_path = "tests/";
    let start_time = Instant::now();

    if let Ok(entries) = fs::read_dir(folder_path) {
        let files: Vec<_> = entries
            .filter_map(|entry| entry.ok().map(|e| e.path()))
            .collect();

        files
            .par_iter()
            .for_each(|file_path| match exec_script(file_path) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!(
                        "\x1b[31mError executing script \x1b[33m{}\x1b[31m: {}\x1b[0m",
                        file_path.display(),
                        e
                    );
                }
            });
    } else {
        return Err(format!("Error reading directory: {}", folder_path).into());
    }

    let elapsed_time = start_time.elapsed();

    println!(
        "\x1b[32mFiles successfully executed in \x1b[33m{}\x1b[32m ms!\x1b[0m",
        elapsed_time.subsec_nanos()
    );

    Ok(())
}
