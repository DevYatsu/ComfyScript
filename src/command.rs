use std::{env::args, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    RunFile(PathBuf),
    MinifyFile(PathBuf),
    NotFound,
    MissingFileName,
}

pub fn get_command() -> Command {
    let mut args = args();
    args.next(); // remove the target/xxx

    let command_name = args.next();

    if let Some(name) = command_name {
        match name.as_str() {
            "minify" => {
                if let Some(file_name) = args.next() {
                    return Command::MinifyFile(file_name.into());
                } else {
                    return Command::MissingFileName;
                }
            }
            file_name => {
                return Command::RunFile(file_name.into());
            }
        }
    }

    return Command::NotFound;
}
