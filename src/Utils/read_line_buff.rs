use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter, Write};
use std::vec;

pub fn read_line_buff(
    params_to_search: &Vec<&str>, // file_to_buffer: &str,
                                  // output_file_result: &str
) -> io::Result<(File)> {
    let path = "search.txt";
    let save_path: &str = "Temp\\ temp_list.txt";

    let mut save_file_handler =
        File::create(save_path)
        .expect("an error was ocurred at read temp file read");

    let file = File::open(path)?;
    let read_buff = BufReader::new(file);

    let lines: Vec<String> = Vec::new();
    for line in read_buff.lines() {
        let line = line?;

        //search all values inside of parms find
        //append all result to final of the file
        for item in params_to_search {
            if line.find(item).is_some() {
                let parsed_str_to_byte = line.replace(item, "");
                let _ = save_file_handler.write_all(parsed_str_to_byte.as_bytes());
            }
        }

        // let mut value_str_to_byte = Vec::new();
        // let str_to_byte = line.as_bytes();
        // for str_byte in str_to_byte {
        // let parse_str_value: Result<String, std::string::FromUtf8Error> =
        // String::from_utf8(vec![*str_byte]);
        // print!("\n");
        // print!("{:?}", parse_str_value.unwrap().remove(0))
        // }

        // println!("line: {}", line)
    }
    Ok(save_file_handler)
}
