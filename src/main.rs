use std::{
    io::{self, },
};

use crate::file_search::create_fields_to_search;


mod readers;
mod file_search;
fn main() {
    println!("entry with your text:\n");

    //gets user input from a comma-separated file
    //and saves it to the array
    //now using fixed string

    // let mut user_input = String::new();
    // io::stdin().read_line(&mut user_input);
    // println!("{}", user_input);

    let fields = ["test", "some thing", "like this"];

    //string fixed value
    let file_path: &str = "Temp\\test.txt";

    // let readr = readers::create_fields_to_search(&fields);
    let value = file_search::create_fields_to_search(&file_path, &["test", "some thing", "like this"]);

    let result: String = match create_fields_to_search(&file_path, &fields) {
        Ok(resul) => resul,
        Err(err) => panic!("not was found: {}", err),
    };
    // match readers::reader("Files\\test.txt") {
    //     Ok(text) => println!("{}", text),
    //     Err(err) => eprintln!("Erro at read file: {}", err),
    // }
}
