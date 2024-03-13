use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::vec;

pub fn read_line_buff(// file_to_buffer: &str
) -> io::Result<Vec<String>> {
    let path = "search.txt";
    let file = File::open(path)?;
    let read_buff = BufReader::new(file);

    let parms_find = vec!["aaa", "testessssss", "de"];

    let lines = Vec::new();
    for line in read_buff.lines() {
        let line = line?;

        //search all values inside of parms find
        let mut iterator = 0;
        for item in &parms_find {
            if line.find(parms_find[iterator]).is_some() {
                println!("{:?}", line.replace(parms_find[0], ""));
                print!("{}", iterator)
            }
            iterator += 1;
            
            
        }
        
        // let mut value_str_to_byte = Vec::new();
        let str_to_byte = line.as_bytes();
        for str_byte in str_to_byte {
            let parse_str_value: Result<String, std::string::FromUtf8Error> =
                String::from_utf8(vec![*str_byte]);
            print!("\n");
            print!("{:?}", parse_str_value.unwrap().remove(0))
        }

        // println!("line: {}", line)
    }
    Ok(lines)
}
