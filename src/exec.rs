use std::{error::Error, path::Path};

use crate::{
    get_file_content,
    parser::{ast, parse_input, Span},
};

pub fn exec_script(path: &Path) -> Result<(), Box<dyn Error>> {
    let content = get_file_content(&path)?;

    if content.is_empty() {
        return Ok(());
    }

    let span = Span::new(content.as_str());
    let (rest, program) = match parse_input(span) {
        Ok(r) => r,
        Err(e) => {
            println!("{}", e);
            return Err("An error occurred!".into());
        }
    };

    if rest.fragment().len() != 0 {
        return Err("Something went wrong! Input is not empty after parsing!".into());
    }

    let program = match program {
        ast::ASTNode::Program { body } => body,
        _ => unreachable!(),
    };

    for node in program {
        println!("{:?}", node);
    }

    Ok(())
}
