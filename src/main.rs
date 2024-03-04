use std::error;

mod readers;
fn main() {
    println!("Hello, world!");

    match readers::reader("Files\\test.txt") {
        Ok(text) => println!("{}", text),
        Err(err) => eprintln!("Erro ao ler o arquivo: {}", err),
    }
}