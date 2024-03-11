use std::fs::File;
use std::io::{self, Read};

pub fn read_file_content(file_path: &str) -> io::Result<String> {
    let mut open_file: File = File::open(file_path)?;

    let mut buffer_content_string = String::new();

    open_file.read_to_string(&mut buffer_content_string)?;

    Ok(buffer_content_string)
}
