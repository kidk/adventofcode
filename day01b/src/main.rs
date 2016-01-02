use std::io::prelude::*;
use std::fs::File;
use std::error::Error;

fn main() {

    let mut buffer = String::new();
    read_file("input.txt", &mut buffer);

    // Loop chars
    let mut count: i32 = 0;
    let mut position: i32 = 0;
    for c in buffer.chars() {
        position += 1;
        match c {
            '(' => { count += 1 }
            ')' => { count -= 1 }
            _ => { }
        }

        if count == -1 {
            break;
        }
    }

    print!("{}", position)
}

fn read_file(filename: &str, buffer: &mut String) {
    // Open file
     let mut file = match File::open(filename) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", filename,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    // Read content
    match file.read_to_string(buffer) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't read {}: {}", filename,
                                                   Error::description(&why)),
        Ok(file) => file,
    };
}
