use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(args);
    // println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);

    //TODO: Add argument checks to ensure this is an argument, and this file exists
    let filepath = &args[1];
    let contents = fs::read_to_string(filepath).expect("File should exist");

    let mut nums: Vec<i32> = vec![];
    for num in contents.split_whitespace() {
        nums.push(num.parse::<i32>().unwrap());
    }

    for n in nums {
        println!("{}", n);
    }

    // println!("File contents:\n{}", nums);
}
