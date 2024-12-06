use std::collections::HashMap;

use itertools::Itertools;

type Point = (usize, usize);
struct Map {
    board: Vec<char>,
    rows: usize,
    cols: usize,

    guard_pos: Point,
    guard_direction: Direction,
    steps: usize,

    visited: HashMap<Point, ()>,
}

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn pos2point(pos: usize, board_width: usize) -> Point {
    let y = pos / board_width;
    let x = pos % board_width;

    (x, y)
}

impl Map {
    fn FromString<T>(input: T) -> Self
        where T: Into<String> 
    {
        let map_str: String = input
            .into();

        let board: Vec<char> = map_str
            .lines()
            .clone()
            .join("")
            .chars()
            .collect();

        let rows = map_str
            .lines()
            .count();

        let cols = map_str
            .lines()
            .next()
            .unwrap()
            .len();

        let guard_pos = board
            .iter()
            .position(|&c| c == '^')
            .unwrap();

        return Self{
            board: board,
            rows: rows,
            cols: cols,

            guard_direction: Direction::Up,
            guard_pos: pos2point(guard_pos, cols),
            steps: 0,
            visited: HashMap::new(),
        }
    }

    fn step(&mut self) {
        if self.is_obstruction_in_front_of_guard() {
            self.guard_direction = match self.guard_direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };

            // println!("NEW DIRECTION({} steps)", self.steps);
            // println!("{}", self.to_string());

            return
        }

        self.steps += 1;
        self.guard_pos = self.next_guard_pos();
        self.visited.insert(self.next_guard_pos(), ());
    }

    fn next_guard_pos(&self) -> Point {
        match self.guard_direction {
            Direction::Up => (self.guard_pos.0, self.guard_pos.1-1),
            Direction::Right => (self.guard_pos.0+1, self.guard_pos.1),
            Direction::Down => (self.guard_pos.0, self.guard_pos.1+1),
            Direction::Left => (self.guard_pos.0-1, self.guard_pos.1),
        }
    }

    fn distinct_visited(&self) -> usize {
        self.visited.len()
    }

    fn is_last_step(&self) -> bool {
        self.guard_pos.0 == 0 
            || self.guard_pos.0 == self.cols - 1
            || self.guard_pos.1 == 0
            || self.guard_pos.1 == self.rows - 1
    }

    fn is_obstruction_in_front_of_guard(&self) -> bool {
        let front_point: Point = self.next_guard_pos();
        // println!("{:?}", front_point);
        return self.board.get(self.point2pos(front_point)).unwrap().to_owned() == '#'
    }

    fn point2pos(&self, p: Point) -> usize {
        p.0 + p.1*self.cols
    }

    fn pos2point(&self, pos: usize) -> Point {
        pos2point(pos, self.cols)
    }

    fn to_string(&self) -> String {
        let dir_indicator = match self.guard_direction {
            Direction::Up => '^',
            Direction::Right => '>',
            Direction::Down => 'v',
            Direction::Left => '<',
        };

        self.board
            .iter()
            .enumerate()
            .map(|(pos, &c)| {
                if self.pos2point(pos) == self.guard_pos {
                    dir_indicator
                } else if c == '^' {
                    '.'
                } else {
                    c
                }
            })
            .enumerate()
            .fold(String::new(), |acc, (i, c)| {
                if i != 0 && i % self.cols == 0{
                    format!("{}\n{}", acc, c)
                } else {
                    format!("{}{}", acc, c)
                }
            })
    }
}


pub fn process_part1(input: &str) -> String {
    let mut map = Map::FromString(input);

    while !map.is_last_step() {
        map.step()
    }

    
   map.distinct_visited().to_string()
}

pub fn process_part2(input: &str) -> String {
   "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}

