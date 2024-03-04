use std::fs;
use std::io::{self, Read};

pub fn reader(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut current_content_string = String::new();
    
    file.read_to_string(&mut current_content_string)?;
    
    Ok(current_content_string)
}