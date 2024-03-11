use std::fs:: File;
use std::io::{self, Read, };

mod read_file;

fn main() {

        match crate::read_file::read_file_content(&"search.txt"){
                Ok(content) => println!("{}", content), 
                Err(err) => eprint!("{err}"),
        }
}
