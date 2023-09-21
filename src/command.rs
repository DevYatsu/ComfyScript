use std::{env::args, path::PathBuf};

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    RunFile(PathBuf),
    NotFound,
}

pub fn get_command() -> Command {
    let mut args = args();
    let file_name = args.next_back();

    if let None = file_name {
        return Command::NotFound;
    }

    if let Some(arg) = args.next_back() {
        if arg == "run" {
            return Command::RunFile(file_name.unwrap().into());
        }
    }

    return Command::NotFound;
}
