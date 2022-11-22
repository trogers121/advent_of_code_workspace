use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    //TODO: Add argument checks to ensure this is an argument, and this file exists
    let filepath = &args[1];
    let contents = fs::read_to_string(filepath).expect("File should exist");

    let mut nums: Vec<i32> = vec![];
    for num in contents.split_whitespace() {
        nums.push(num.parse::<i32>().unwrap());
    }
    let mut larger_cnt: i32 = 0;
    for i in 1..(nums.len()) {
        if nums[i] > nums[i - 1] {
            larger_cnt += 1;
        }
    }

    println!("{}", larger_cnt);
}
