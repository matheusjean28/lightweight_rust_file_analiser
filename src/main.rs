use std::fs::File;
use std::io::{self, Read};
use std::process;

mod Utils {
    pub mod read_file;
    pub mod save_file; 
    pub mod read_line_buff;
}
use Utils::{read_file, save_file, read_line_buff};
// mod save_file;
fn main() {
    let content: String;

    let content: String = match Utils::read_file::read_file_content(&"search.txt") {
        Ok(content) => {
            println!("Was read here: {}", content);
            content
        }
        Err(err) => {
            eprintln!("Err at read file: {}", err);
            process::exit(1);
        }
    };

    match save_file::save_file(&content) {
        Ok(file) => {
            println!("File saved: {:?}", file);
        }
        Err(err) => {
            eprintln!("Err at save file: {}", err);
            process::exit(1);
        }
    };

    match  read_line_buff::read_line_buff() {
        Ok(content) => print!("{:?}", content),
        Err(err) => eprint!("{}", err),
    }

}
