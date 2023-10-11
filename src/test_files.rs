use std::{error::Error, fs};

pub fn parse_all_files() -> Result<(), Box<dyn Error>> {
    use crate::exec::exec_script;

    let folder_path = "tests/";

    if let Ok(entries) = fs::read_dir(folder_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let file_path = entry.path();

                // Use match to handle the Result returned by exec_script
                match exec_script(&file_path) {
                    Ok(_) => { /* Successfully executed script */ }
                    Err(e) => eprintln!("Error executing script {}: {}", file_path.display(), e),
                }
            } else {
                eprintln!("Error accessing directory entry");
            }
        }
    } else {
        eprintln!("Error reading directory: {}", folder_path);
    }

    Ok(())
}
