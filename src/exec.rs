use std::{error::Error, path::Path};

use crate::{
    get_file_content,
    parser::{parse_input, Span},
};

pub fn exec_script(path: &Path) -> Result<(), Box<dyn Error>> {
    let content = get_file_content(&path)?;

    let span = Span::new(content.as_str());
    let (rest, _ast_node) = match parse_input(span) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return Err("An error occurred!".into());
        }
    };

    if rest.fragment().len() != 0 {
        return Err("Something went wrong! Input is not empty after parsing!".into());
    }

    Ok(())
}
