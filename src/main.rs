use std::{
    error,
    io::{self, stdin, Read},
};


mod readers;
fn main() {

    println!("entry with your text:\n");
    
    //gets user input from a comma-separated file 
    //and saves it to the array
    //now using fixed string
    
    let mut user_input = String::new();
    io::stdin().read_line(&mut user_input);
    println!("{}", user_input);

    let mut fields = ["test", "some thing", "like this"];
    

    readers::create_fields_to_search("\\search.txt",&fields);

    // match readers::reader("Files\\test.txt") {
    //     Ok(text) => println!("{}", text),
    //     Err(err) => eprintln!("Erro at read file: {}", err),
    // }
}
