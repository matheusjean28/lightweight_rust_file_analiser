use std::fs::{self, File};
use std::io::{self, Write};

pub fn save_file(_content: &String) -> io::Result<File> {
    let path: String = String::from("Temp\\ temp_list.txt");
    if !fs::metadata(path.clone()).is_ok() {
        println!("file does not exist, creating it!");
        File::create("Temp\\temp_list.txt")?;
    }

    let mut file_content = File::create(path)?;

    let mut content = String::new();

    let text_to_write = "teste,teste";

    file_content.write_all(_content.as_bytes())?;

    println!("{:?}", file_content);

    Ok(file_content)
}
