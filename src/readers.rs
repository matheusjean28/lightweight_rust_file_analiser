use std::fs::{self, OpenOptions};
use std::fs::File;
use std::io::{self, Read, Write};

//creat a temp file if not exist and save fields at them
pub fn create_fields_to_search(file_to_read_path: &str, array_params: &[&str]) -> io::Result<String> {
    println!("File to read path: {}", file_to_read_path);

    
    let temp_file_path = "Temp/temp_list.txt";

    if !std::path::Path::new(temp_file_path).exists() {
        // Create file if it doesn't exist
        let mut file = File::create(temp_file_path)?;
        
        // Write to file
        for field in array_params {
            writeln!(file, "{}", field)?;
        }
    } 

    let mut file = File::open(file_to_read_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

//read file with io stream

pub fn reader(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut current_content_string = String::new();

    file.read_to_string(&mut current_content_string)?;

    Ok(current_content_string)
}
