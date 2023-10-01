use std::{error::Error, path::Path};

use crate::parser::parse_input;

pub fn minify(content: &'static str, file_path: &Path) -> Result<(), Box<dyn Error>> {
    let (rest, ast_node) = parse_input(content.into(), None)?;

    if rest.fragment().len() != 0 {
        println!("Failed to minify: parsing step did not return empty input");
        std::process::exit(0);
    }

    let mut buffer = String::new();

    for node in ast_node {
        buffer.push_str(&node.to_string())
    }
    Ok(())
}
