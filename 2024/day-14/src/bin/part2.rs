use std::fs;

use day_14::process_part2;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", process_part2(&input, 101, 103));
}