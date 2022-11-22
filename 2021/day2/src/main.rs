use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    //TODO: Add argument checks to ensure this is an argument, and this file exists
    let filepath = &args[1];
    let contents = fs::read_to_string(filepath).expect("File should exist");

    // Read the file input. Now we need to break it up into tokens.
    let mut depth: i32 = 0;
    let mut pos: i32 = 0;
    let mut aim: i32 = 0;
    for cmd in contents.split('\n') {
        let action: Vec<&str> = cmd.split(' ').collect();
        let dir: &str = action[0];
        let mag: i32 = action[1].parse().unwrap();

        match dir {
            "up" => {
                aim -= mag;
            }
            "down" => {
                aim += mag;
            }
            "forward" => {
                pos += mag;
                depth += aim * mag;
            }
            _ => panic!("Unknown direction!"),
        }
    }

    println!("Vector = {}", depth * pos);
}
