use std::path::Path;
use std::io;
use std::io::prelude::*;
use std::fs::File;

pub fn get_file_contents(filename: &str) -> Result<String, io::Error> {
    let path = Path::new(filename);
    let mut file = File::open(&path)?;

    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

pub fn write_to_file(filename: &str, content: &str) -> Result<File, io::Error> {
    let path = Path::new(filename);
    let mut file = File::create(path)?;

    file.write_all(content.as_bytes())?;

    Ok(file)
}
