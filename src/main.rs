use std::fs::File;
use std::io::{self, Read};
use std::process;

mod Utils {
    pub mod read_file;
    pub mod read_line_buff;
    pub mod save_file;
}
use Utils::{read_file, read_line_buff, save_file};
// mod save_file;
fn main() {
    let _content: String;

    // let content: String = match Utils::read_file::read_file_content(&"search.txt") {
    //     Ok(content) => {
    //         println!("Was read here: {}", content);
    //         content
    //     }
    //     Err(err) => {
    //         eprintln!("Err at read file: {}", err);
    //         process::exit(1);
    //     }
    // };

    // match save_file::save_file(&content) {
    //     Ok(file) => {
    //         println!("File saved: {:?}", file);
    //     }
    //     Err(err) => {
    //         eprintln!("Err at save file: {}", err);
    //         process::exit(1);
    //     }
    // };

    //vec of params as str to search
    let parms_find = vec!["aaa", "a", "de", "matheus", "teste123"];

    match read_line_buff::read_line_buff(&parms_find) {
        Ok(content) => {
            println!("{:?}", content)
        }
        Err(err) => eprint!("{}", err),
    }
}
