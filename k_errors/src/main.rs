mod propagating;

use std::{fs::File, io::ErrorKind, io::Write};

fn main() {
    let open_file_result = File::open("hello.txt");

    let mut open_file = match open_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {

            ErrorKind::NotFound => {
                match File::create("hello.txt") {

                    Ok(fc) => fc,
                    Err(e) => panic!("Problem creating a file: {:?}", e)
                }
            },

            _ => panic!("Encountered an error: {:?}", error)
        }
    };

    open_file.write_all(b"jarvis").expect("Cannot write file");
    println!("The username: {:#?}", propagating::read_username());
    println!("The username: {:#?}", propagating::read_username_optimized());
}

