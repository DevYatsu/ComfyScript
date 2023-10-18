use std::path::Path;

use nom_supreme::final_parser::Location;

use crate::{
    errors::ComfyScriptError,
    get_file_content,
    parser::{ast, parse_input},
};

pub fn exec_script(path: &Path) -> Result<(), ComfyScriptError> {
    let content = get_file_content(&path)?;

    if content.is_empty() {
        return Ok(());
    }

    let program = match parse_input(&content) {
        Ok(r) => r,
        Err(e) => match e {
            nom_supreme::error::GenericErrorTree::Stack { contexts, .. } => {
                let ctx = contexts[0].1;
                let location = Location::locate_tail(&content, &contexts[0].0);

                match ctx {
                    nom_supreme::error::StackContext::Context(msg) => {
                        let error = ComfyScriptError::ParsingFailed {
                            input: content.to_owned(),
                            advice: msg.to_string(),
                            message: (location.line, location.column).into(),
                        };

                        return Err(error);
                    }
                    _ => unreachable!(),
                }
            }
            _ => unreachable!(),
        },
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
