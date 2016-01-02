use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::io::BufReader;
use std::cmp;

fn main() {
    let reader = read_file("input.txt");
    let result = parse(reader);

    println!("{}", result)
}

fn parse(reader: BufReader<File>) -> i32 {
    let mut total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() > 1 {
            total += calculate(line);
        }
    }

    total
}

fn calculate(line: String) -> i32 {
    let s: Vec<&str> = line.split('x').collect();

    let l = s[0].parse::<i32>().unwrap();
    let w = s[1].parse::<i32>().unwrap();
    let h = s[2].parse::<i32>().unwrap();

    let sidea = l * w;
    let sideb = w * h;
    let sidec = h * l;

    let mut needed = cmp::min(sidea, sideb);
    needed = cmp::min(needed, sidec);

    needed + (2 * sidea) + (2 * sideb) + (2 * sidec)
}

fn read_file(filename: &str) -> BufReader<File> {
    // Open file
    let file = match File::open(filename) {
        // The `description` method of `io::Error` returns a string that
        // describes the error
        Err(why) => panic!("couldn't open {}: {}", filename,
                                                   Error::description(&why)),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    reader
}
