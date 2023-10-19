use codespan_reporting::{diagnostic::Label, files::SimpleFile};
use nom::{character::complete::alphanumeric1, combinator::map, error::{VerboseError, Error}, Err};
use nom_supreme::{final_parser::Location, error::GenericErrorTree};

use crate::parser::{ast, errors::SyntaxError, parse_input};

pub fn exec_script(
    content: String,
) -> Result<(), (SyntaxError<()>, SimpleFile<&'static str, String>)> {
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
                        let file = SimpleFile::new("Test.cfs", content.to_owned());
                        let error_place = content.len() - contexts[0].0.len();
                        let error_length = alphanumeric1::<&str, Error<&str>>(contexts[0].0).and_then(|(_, w)| Ok(w.len()) ).unwrap_or(1);
                        
                        let found = &contexts[0].0[0..error_length];
                        
                        let mut err = SyntaxError::identifier(found);
                        err.add_label(Label::primary((), error_place..error_place + error_length));

                        return Err((err, file));
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
