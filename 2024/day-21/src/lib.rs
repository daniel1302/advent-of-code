#![feature(iter_map_windows)]

use lazy_static::lazy_static;
use std::{collections::BTreeMap, i32};

mod aoclib;
use aoclib::*;
type Keyboard = BTreeMap<char, Vec2>;

lazy_static! {
    // +---+---+---+
    // | 7 | 8 | 9 |
    // +---+---+---+
    // | 4 | 5 | 6 |
    // +---+---+---+
    // | 1 | 2 | 3 |
    // +---+---+---+
    //     | 0 | A |
    //     +---+---+
    static ref NUM_PAD: Keyboard = BTreeMap::from([
        ('7', Vec2{x: 0, y: 0}),
        ('8', Vec2{x: 1, y: 0}),
        ('9', Vec2{x: 2, y: 0}),
        ('4', Vec2{x: 0, y: 1}),
        ('5', Vec2{x: 1, y: 1}),
        ('6', Vec2{x: 2, y: 1}),
        ('1', Vec2{x: 0, y: 2}),
        ('2', Vec2{x: 1, y: 2}),
        ('3', Vec2{x: 2, y: 2}),
        (' ', Vec2{x: 0, y: 3}),
        ('0', Vec2{x: 1, y: 3}),
        ('A', Vec2{x: 2, y: 3}),
    ]);


    //     +---+---+
    //     | ^ | A |
    // +---+---+---+
    // | < | v | > |
    // +---+---+---+
    static ref ARROW_PAD: Keyboard = BTreeMap::from([
        (' ', (0, 0).into()),
        ('^', (1, 0).into()),
        ('A', (2, 0).into()),
        ('<', (0, 1).into()),
        ('v', (1, 1).into()),
        ('>', (2, 1).into()),
    ]);

    static ref NUM_PAD_GRID: BTreeMap<Vec2, char> = BTreeMap::from([
        (Vec2{x: 0, y: 0}, '7'),
        (Vec2{x: 1, y: 0}, '8'),
        (Vec2{x: 2, y: 0}, '9'),
        (Vec2{x: 0, y: 1}, '4'),
        (Vec2{x: 1, y: 1}, '5'),
        (Vec2{x: 2, y: 1}, '6'),
        (Vec2{x: 0, y: 2}, '1'),
        (Vec2{x: 1, y: 2}, '2'),
        (Vec2{x: 2, y: 2}, '3'),
        (Vec2{x: 0, y: 3}, ' '),
        (Vec2{x: 1, y: 3}, '0'),
        (Vec2{x: 2, y: 3}, 'A'),
    ]);

    static ref ARROW_GRID: BTreeMap<Vec2, char> = BTreeMap::from([
        (Vec2{x: 0, y: 0}, ' '),
        (Vec2{x: 1, y: 0}, '^'),
        (Vec2{x: 2, y: 0}, 'A'),
        (Vec2{x: 0, y: 1}, '<'),
        (Vec2{x: 1, y: 1}, 'v'),
        (Vec2{x: 2, y: 1}, '>'),
    ]);

}

#[derive(Debug)]
struct Code<'a> {
    code: &'a str,
    num: i32,
}

fn parse_input(input: &str) -> Vec<Code<'_>> {
    input
        .lines()
        .map(|num| {
            Code{
                code: num,
                num: str_to_int(num),
            }
        })
        .collect()
}


fn available_paths_new(keyboard_grid: &BTreeMap<Vec2, char>, from_pos: Vec2, to_pos: Vec2) -> Vec<String> {
    if !keyboard_grid.contains_key(&from_pos) || keyboard_grid[&from_pos] == ' ' {
        return Vec::new();
    }

    if from_pos == to_pos {
        return vec!["A".to_string()];
    }

    let mut paths = vec![];
    if from_pos.y > to_pos.y {
        for path in available_paths_new(keyboard_grid, Vec2 { x: from_pos.x, y: from_pos.y-1 }, to_pos) {
            paths.push(["^".to_string(), path].join(""));
        }
    }
    if from_pos.y < to_pos.y {
        for path in available_paths_new(keyboard_grid, Vec2 { x: from_pos.x, y: from_pos.y+1 }, to_pos) {
            paths.push(["v".to_string(), path].join(""));
        }
    }
    if from_pos.x > to_pos.x {
        for path in available_paths_new(keyboard_grid, Vec2 { x: from_pos.x - 1, y: from_pos.y }, to_pos) {
            paths.push(["<".to_string(), path].join(""));
        }
    }
    if from_pos.x < to_pos.x {
        for path in available_paths_new(keyboard_grid, Vec2 { x: from_pos.x + 1, y: from_pos.y }, to_pos) {
            paths.push([">".to_string(), path].join(""));
        }
    }

    paths
}


fn shortest_path_at_layer(from_c: char, to_c: char, layer: i32, first_layer: bool, cache: &mut BTreeMap<(char, char, i32), u128>) -> u128 {
    if layer == 0 {
        return 1;
    }

    let cache_key = (from_c, to_c, layer);
    if cache.contains_key(&cache_key) {
        return cache.get(&cache_key).unwrap().to_owned();
    }

    let keyboard: &Keyboard = match first_layer {
        false => &ARROW_PAD,
        true => &NUM_PAD,
    };

    
    let keyboard_grid: &BTreeMap<Vec2, char> = match first_layer {
        false => &ARROW_GRID,
        true => &NUM_PAD_GRID,
    };


    // let cache_key = ()
    let paths_between_keys = available_paths_new(keyboard_grid, keyboard[&from_c], keyboard[&to_c]);

    let mut shortest_path = u128::MAX;
    for path in paths_between_keys {
        let path = ["A", &path].join("");

        let sum = path
            .chars()
            .map_windows(|&[from_c, to_c]| {
                shortest_path_at_layer(from_c, to_c, layer-1, false, cache)
            })
            .sum::<u128>();

        shortest_path = shortest_path.min(sum);
    }

    cache.insert(cache_key, shortest_path);
    shortest_path
}



fn solve_code(code: &str, depth: i32) -> u128 {
    let code = ["A", code].join("");

    let mut cache = BTreeMap::new();
    code
        .chars()
        .map_windows(|&[from_c, to_c]| {
            shortest_path_at_layer(from_c, to_c, depth, true, &mut cache)
        })
        .sum::<u128>()
}

pub fn process_part1(input: &str) -> String {
    let codes = parse_input(input);

    codes.iter().map(|c: &Code<'_>| {
        let res = solve_code(c.code, 3);
        res * c.num as u128
    }).sum::<u128>()
    .to_string()
}

pub fn process_part2(input: &str) -> String {
    
    let codes = parse_input(input);

    codes.iter().map(|c: &Code<'_>| {
        let res = solve_code(c.code, 26);
        res * c.num as u128
    }).sum::<u128>()
    .to_string()

}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}