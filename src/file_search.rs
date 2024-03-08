
    use std::{
        fs::File,
        io::{self, Read},
        io::Write
    };


    pub fn create_fields_to_search(
        file_to_read_path: &str,
        array_params: &[&str],
    ) -> io::Result<<T> io::Error> {
        println!("File to read path: {}", file_to_read_path);

        let temp_file_path: &str = "Temp/temp_list.txt";
        

        if !std::path::Path::new(temp_file_path).exists() {
            // Create file if it doesn't exist
            let mut file = File::create(temp_file_path)?;

            // Write to file
            for field in array_params {
                writeln!(file, "{}", field)?;
            }
        }

        let file_path: Result<File, io::Error> = Ok(File::open(file_to_read_path).expect("error:"));
        let mut read_file = match file_path {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {:?}", error),
        };

        let mut contents = String::new();
        Ok(read_file.read_to_string(&mut contents));

    }
