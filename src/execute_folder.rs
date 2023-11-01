use std::{
    error::Error,
    fs,
    path::Path,
    sync::{Arc, Mutex},
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::{get_file_content, script::ComfyScript};

pub fn execute_folder(folder_path: &Path) -> Result<(), Box<dyn Error>> {
    let files = fs::read_dir(folder_path)
        .map_err(|e| {
            eprintln!("Error reading directory: {}", e);
            Box::new(e) as Box<dyn Error>
        })?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect::<Vec<_>>();

    let errors = Arc::new(Mutex::new(vec![]));

    files.par_iter().for_each(|file_path| {
        let content = match get_file_content(file_path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!(
                    "\x1b[31mError\x1b[33m{}\x1b[31m: {}\x1b[0m",
                    file_path.display(),
                    e
                );
                        

                return;
            }
        };

        let script = ComfyScript::new(file_path.to_string_lossy(), content);
        if let Err(err_data) = script.execute() {
            let mut errors_list = errors.lock().unwrap();
            errors_list.push(err_data);
        } else {
            println!(
                "\x1b[33m{}\x1b[32m successfully executed!\x1b[0m",
                file_path.display()
            )
        }
    });

    let errors = Arc::try_unwrap(errors).expect("Failed to unwrap errors Arc");
    let errors_list = errors.into_inner().unwrap();

    for (err_data, file) in errors_list {
        err_data.print(file).unwrap();
    }
    Ok(())
}
