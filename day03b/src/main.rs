use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::cmp;
use std::iter::repeat;


fn main() {

    let mut buffer = String::new();
    read_file("input.txt", &mut buffer);

    // Find size of grid
    let mut x_min: i32 = -100;
    let mut y_min: i32 = -100;
    let mut x_current: i32 = 0;
    let mut x_max: i32 = 100;
    let mut y_max: i32 = 100;
    let mut y_current: i32 = 0;

    let width = (x_min.abs() + x_max) as usize;
    let height = (y_min.abs() + y_max) as usize;

    let mut x_santa: usize = x_min.abs() as usize;
    let mut y_santa: usize = y_min.abs() as usize;

    let mut x_robot = x_santa;
    let mut y_robot = y_santa;

    // Create vector array of right size
    let mut state: Vec<u32> = repeat(0u32).take(height * width).collect();

    // Set initial location to active
    state[y_santa * height + x_santa] = 2;

    let mut santa = true;

    // Loop path and increment value in array
    for c in buffer.chars() {
        println!("santa {} {}", x_santa, y_santa);
        println!("robot {} {}", x_robot, y_robot);
        if santa {
            // I should move this double code to impl and struct, probably usefull when you many santas
            match c {
                '>' => { x_santa += 1; }
                '<' => { x_santa -= 1; }
                '^' => { y_santa += 1; }
                'v' => { y_santa -= 1; }
                _ => { }
            }
            state[y_santa * height + x_santa] += 1;
        } else {
            match c {
                '>' => { x_robot += 1; }
                '<' => { x_robot -= 1; }
                '^' => { y_robot += 1; }
                'v' => { y_robot -= 1; }
                _ => { }
            }
            state[y_robot * height + x_robot] += 1;
        }

        santa = !santa;
    }

    // Find number of active elements
    let mut counter = 0;
    for i in 0..state.len() {
        if state[i] > 0 {
            counter += 1;
        }
    }

    // Print result
    println!("{}", counter);
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
