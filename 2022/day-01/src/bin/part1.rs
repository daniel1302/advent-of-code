use std::fs;

use day_01::process_part1;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    println!("{}", process_part1(&content))
}