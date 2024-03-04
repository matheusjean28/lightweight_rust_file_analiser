use std::fs;
use std::fs::File;
use std::io::{self, Read, Write};

//creat a temp file if not exist and save fields at them
pub fn create_fields_to_search(file_path_of_search: &str, array_params: &[&str]) -> io::Result<()> {
    if !std::path::Path::new("Temp\\temp_list.txt").exists() {
        
        //create file if not exist
        let mut file = File::create("Temp\\temp_list.txt")?;

        //white at file
        for field in array_params{
            writeln!(file,"{}", field)?;
        }
        
    } else {
        //do not
    };
    
    Ok(())
}

//read file with io stream
pub fn reader(file_path: &str) -> io::Result<String> {
    let mut file = fs::File::open(file_path)?;
    let mut current_content_string = String::new();

    file.read_to_string(&mut current_content_string)?;

    Ok(current_content_string)
}
