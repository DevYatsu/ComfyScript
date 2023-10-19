use nom_supreme::final_parser::Location;

use crate::{
    errors::ComfyScriptError,
    parser::{ast, errors::SyntaxError, parse_input},
};

pub fn exec_script(content: String) -> Result<(), SyntaxError> {
    if content.is_empty() {
        return Ok(());
    }

    let program = match parse_input(&content) {
        Ok(r) => r,
        Err(e) => match e {
            nom_supreme::error::GenericErrorTree::Stack { contexts, .. } => {
                let ctx = contexts[0].1;
                let location = Location::locate_tail(&content, &contexts[0].0);

                // let content: String = content.lines().enumerate().filter(|(i, _)| i>= &(location.line-1) && i <= &(location.line+1)).map(|(_, l)| l).collect();

                match ctx {
                    nom_supreme::error::StackContext::Context(msg) => {
                        let error = ComfyScriptError::ParsingFailed {
                            input: content.clone(),
                            advice: msg.to_string(),
                            message: (location.line, location.column + 5).into(),
                        };

                        let specifierErr = SyntaxError::ExpectedSpecifier {
                            input: content,
                            span: (location.line, location.column + 5).into(),
                        };

                        return Err(specifierErr);
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
