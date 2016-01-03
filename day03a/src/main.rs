use std::io::prelude::*;
use std::fs::File;
use std::error::Error;
use std::cmp;
use std::iter::repeat;

fn main() {

    let mut buffer = String::new();
    read_file("input.txt", &mut buffer);

    // Find size of grid
    let mut x_min: i32 = 0;
    let mut y_min: i32 = 0;
    let mut x_current: i32 = 0;
    let mut x_max: i32 = 0;
    let mut y_max: i32 = 0;
    let mut y_current: i32 = 0;

    for c in buffer.chars() {
        match c {
            '>' => {
                x_current += 1;
                x_max = cmp::max(x_current, x_max);
            }
            '<' => {
                x_current -= 1;
                x_min = cmp::min(x_current, x_min);
            }
            '^' => {
                y_current += 1;
                y_max = cmp::max(y_current, y_max);
            }
            'v' => {
                y_current -= 1;
                y_min = cmp::min(y_current, y_min);
            }
            _ => { }
        }
    }

    let width = (x_min.abs() + x_max) as usize;
    let height = (y_min.abs() + y_max) as usize;

    let mut x: usize = x_min.abs() as usize;
    let mut y: usize = y_min.abs() as usize;

    // Create vector array of right size
    let mut state: Vec<u32> = repeat(0u32).take(height * width).collect();

    // Set initial location to active
    state[y * height + x] = 1;

    // Loop path and increment value in array
    for c in buffer.chars() {
        match c {
            '>' => {
                x += 1;
            }
            '<' => {
                x -= 1;
            }
            '^' => {
                y += 1;
            }
            'v' => {
                y -= 1;
            }
            _ => { }
        }

        state[y * height + x] += 1;
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
