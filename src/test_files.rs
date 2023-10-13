use std::{error::Error, fs, time::Instant};

pub fn parse_all_files() -> Result<(), Box<dyn Error>> {
    use crate::exec::exec_script;

    let folder_path = "tests/";
    let start_time = Instant::now();

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();

                // Use match to handle the Result returned by exec_script
                match exec_script(&file_path) {
                    Ok(_) => {}
                    Err(e) => {
                        return Err(format!(
                            "\x1b[31mError executing script \x1b[33m{}\x1b[31m: {}\x1b[0m",
                            file_path.display(),
                            e
                        )
                        .into())
                    }
                }
            } else {
                return Err("Error accessing directory entry".into());
            }
        }
    } else {
        return Err(format!("Error reading directory: {}", folder_path).into());
    }

    let elapsed_time = start_time.elapsed();

    println!(
        "\x1b[32mFiles successfully executed in \x1b[33m{}\x1b[32m ms!\x1b[0m",
        elapsed_time.as_millis()
    );

    Ok(())
}
