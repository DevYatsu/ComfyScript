use std::{error::Error, fs, path::Path};

pub fn generate_minified_file(initial_path: &Path, content: &[u8]) -> Result<(), Box<dyn Error>> {
    let file_name = initial_path
        .file_name()
        .ok_or("Invalid file path!")?
        .to_string_lossy()
        .to_string();

    let parent_dir = initial_path.parent().unwrap_or_else(|| Path::new(""));

    let new_path = parent_dir.join(format!("minified.{}", file_name));

    fs::write(new_path, content)?;

    Ok(())
}
