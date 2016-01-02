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

    let first = cmp::min(l, w);
    let second = cmp::min(cmp::max(l, w), h);
    let ribbon = first * 2 + second * 2;

    ribbon + (l * w * h)
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

#[test]
fn check() {
    assert!(calculate(String::from("22x19x18")) == 7598);
    assert!(calculate(String::from("13x1x24")) == 340);
}
