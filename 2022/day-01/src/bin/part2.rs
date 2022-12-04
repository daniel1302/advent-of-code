use std::fs;

use day_01::process_part2;

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    println!("{}", process_part2(&content))
}