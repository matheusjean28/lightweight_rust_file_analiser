use std::fs::File;
use std::io::{self, BufRead, BufReader};

pub fn read_line_buff(// file_to_buffer: &str
) -> io::Result<Vec<String>> {
    let path = "search.txt";
    let file = File::open(path)?;
    let read_buff = BufReader::new(file);

    let mut lines = Vec::new();
    for line in read_buff.lines() {
        let line = line?;
        println!("line: {}", line)
    }
    Ok(lines)
}
