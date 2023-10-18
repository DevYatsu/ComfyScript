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
            match e {
                nom_supreme::error::GenericErrorTree::Base { location, kind } => {
                    println!("its base")
                }
                nom_supreme::error::GenericErrorTree::Stack { base, contexts } => {
                    println!("its stack")
                }
                nom_supreme::error::GenericErrorTree::Alt(val) => match &val[val.len() - 1] {
                    nom_supreme::error::GenericErrorTree::Stack { base, contexts } => {
                        println!("{:?}", contexts[contexts.len() - 1]);
                        println!("{:?}", *base);
                    }
                    _ => {
                        println!("here: {:?}", val[val.len() - 1])
                    },
                },
            }

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
