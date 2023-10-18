use std::{error::Error, path::Path};

use crate::{
    get_file_content,
    parser::{ast, parse_input},
};

pub fn exec_script(path: &Path) -> Result<(), Box<dyn Error>> {
    let content = get_file_content(&path)?;

    if content.is_empty() {
        return Ok(());
    }

    let program = match parse_input(&content) {
        Ok(r) => r,
        Err(e) => {
            println!("err {:?}", e);
            // todo! manage error here to display a user friendly message

            return Err("An error occurred!".into());
        }
    };

    let program = match program {
        ast::ASTNode::Program { body } => body,
        _ => unreachable!(),
    };

    for node in program {
        println!("{:?}", node);
    }

    Ok(())
}
