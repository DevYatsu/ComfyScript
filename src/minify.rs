use std::{error::Error, fs, path::Path};

use crate::{
    get_file_content,
    parser::{ast, parse_input, Span},
};

pub fn minify_input(path: &Path) -> Result<(), Box<dyn Error>> {
    let content = get_file_content(&path)?;

    if content.is_empty() {
        return Ok(generate_minified_file(path, &[])?);
    }

    let span = Span::new(content.as_str());
    let (rest, program) = match parse_input(span) {
        Ok(r) => r,
        Err(_) => return Err("An error occurred!".into()),
    };

    if rest.len() != 0 {
        return Err("Something went wrong! Input is not empty after parsing!".into());
    }

    let mut buffer = String::new();

    let program = match program {
        ast::ASTNode::Program { body } => body,
        _ => unreachable!(),
    };

    for node in program {
        buffer.push_str(&node.to_string())
    }

    generate_minified_file(path, buffer.as_bytes())?;

    Ok(())
}

fn generate_minified_file(initial_path: &Path, content: &[u8]) -> Result<(), Box<dyn Error>> {
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
