use std::fs;

use day_24::process_part1;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part1(&input));
}