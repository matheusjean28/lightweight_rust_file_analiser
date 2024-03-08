use std::fs::File;
use std::fs::{self};
use std::io::{self, Read, Write};

//creat a temp file if not exist and save fields at them


//read file with io stream

pub fn reader(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut current_content_string = String::new();

    file.read_to_string(&mut current_content_string)?;

    Ok(current_content_string)
}
