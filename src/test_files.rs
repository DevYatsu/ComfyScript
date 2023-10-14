use std::{
    error::Error,
    fs,
    sync::{Arc, Mutex},
    time::Instant,
};

use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};

use crate::exec::exec_script;

pub fn parse_all_files() -> Result<(), Box<dyn Error>> {
    let folder_path = "tests/";
    let start_time = Instant::now();

    let files = fs::read_dir(folder_path)
        .map_err(|e| {
            eprintln!("Error reading directory: {}", e);
            Box::new(e) as Box<dyn Error>
        })?
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect::<Vec<_>>();

    let errors_list = Arc::new(Mutex::new(vec![]));

    files.par_iter().for_each(|file_path| {
        if let Err(e) = exec_script(file_path) {
            let mut list = errors_list.lock().unwrap();
            list.push(format!(
                "\x1b[31mError executing script \x1b[33m{}\x1b[31m: {}\x1b[0m",
                file_path.display(),
                e
            ));
        }
    });

    let errors_list = Arc::try_unwrap(errors_list)
        .expect("Failed to unwrap Arc")
        .into_inner()
        .unwrap();

    for error in errors_list {
        eprintln!("{}", error);
    }

    let elapsed_time = start_time.elapsed();

    println!(
        "\x1b[32mFiles successfully executed in \x1b[33m{:?}\x1b[32m!\x1b[0m",
        elapsed_time.as_nanos()
    );

    Ok(())
}
