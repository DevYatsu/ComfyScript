// handling filesystem, paths and etct

use std::{fs, io::Error};

pub fn _read_file(file_path: &str) -> Result<String, Error> {
    fs::read_to_string(file_path)
}
