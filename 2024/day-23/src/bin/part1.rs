use std::fs;

use day_23::process_part1;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part1(&input));
}