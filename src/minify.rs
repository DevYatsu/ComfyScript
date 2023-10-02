use std::{error::Error, fs, path::Path};

use crate::{
    get_file_content,
    parser::{parse_input, Span},
};

pub fn minify_input(path: &Path) -> Result<(), Box<dyn Error>> {
    let content = get_file_content(&path)?;

    let span = Span::new(content.as_str());
    let (rest, ast_nodes) = match parse_input(span) {
        Ok(r) => r,
        Err(_) => return Err("An error occurred!".into()),
    };

    if rest.len() != 0 {
        return Err("Something went wrong! Input is not empty after parsing!".into());
    }

    let mut buffer = String::new();

    for node in ast_nodes {
        buffer.push_str(&node.to_string())
    }

    let new_path = "minified.".to_owned() + &path.to_string_lossy();

    fs::write(new_path, buffer.as_bytes())?;

    Ok(())
}