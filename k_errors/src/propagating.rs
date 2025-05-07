use std::io::{self, Read};
use std::fs::File;

pub fn read_username() -> Result<String, io::Error> {
    let file_read = File::open("hello.txt");
    
    let mut file = match file_read {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

pub fn read_username_optimized() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;   // the ? operator works when the
                                                               // return type is a Result or Option
    Ok(username)
}
