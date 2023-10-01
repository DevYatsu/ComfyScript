use std::{env::args, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    RunFile(PathBuf),
    NotFound,
    MinifyFile(PathBuf),
}

pub fn get_command() -> Command {
    let mut args = args();
    let file_name = args.next_back();

    if let None = file_name {
        return Command::NotFound;
    }

    if let Some(arg) = args.next_back() {
        match arg.as_str() {
            "run" => return Command::RunFile(file_name.unwrap().into()),
            "minify" => return Command::MinifyFile(file_name.unwrap().into()),
            _ => (),
        }
    }

    return Command::NotFound;
}
