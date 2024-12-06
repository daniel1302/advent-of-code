use std::collections::HashMap;

use itertools::Itertools;

type Point = (i32, i32);

enum SimulationResult {
    Finished(i32),
    Loop(i32),
}

#[derive(Clone)]
struct GuardSimulator {
    board: Vec<char>,
    rows: usize,
    cols: usize,

    guard_pos: Point,
    guard_direction: Direction,

    visited: HashMap<Point, ()>,
    steps: HashMap<(Point, Point), ()>,
}

#[derive(Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn pos2point(pos: usize, board_width: usize) -> Point {
    let y = pos / board_width;
    let x = pos % board_width;

    (x as i32, y as i32)
}


impl GuardSimulator {
    fn from_string<T>(input: T) -> Self
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

        let mut visited = HashMap::new();
        visited.insert(pos2point(guard_pos, cols), ());

        return Self{
            board: board,
            rows: rows,
            cols: cols,

            guard_direction: Direction::Up,
            guard_pos: pos2point(guard_pos, cols),
            steps: HashMap::new(),
            visited: visited,
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

        } else {

            self.steps.insert((self.guard_pos, self.next_guard_pos()), ());
            self.guard_pos = self.next_guard_pos();
            self.visited.insert(self.guard_pos, ());
        }
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

    fn no_more_steps(&self) -> bool {
        let next_pos = self.next_guard_pos();

        return next_pos.0 < 0 
            || next_pos.1 < 0 
            || next_pos.0 >= self.cols as i32 
            || next_pos.1 >= self.rows as i32
    }

    fn is_obstruction_in_front_of_guard(&self) -> bool {
        let front_point: Point = self.next_guard_pos();
        return self.board.get(self.point2pos(front_point)).unwrap().to_owned() == '#'
    }

    fn point2pos(&self, p: Point) -> usize {
        p.0 as usize + p.1 as usize *self.cols
    }

    fn pos2point(&self, pos: usize) -> Point {
        pos2point(pos, self.cols)
    }

    fn is_obstruction_at(&self, pos: usize) -> bool {
        self.board[pos] == '#'
    }

    fn is_guard_at(&self, pos: usize) -> bool {
        self.board[pos] == '^' 
            || self.board[pos] == 'v' 
            || self.board[pos] == '>'
            || self.board[pos] == '<'
    }

    fn put_obstruction(&mut self, pos: usize) {
        self.board[pos] = '#'
    }

    fn is_guard_in_loop(&self) -> bool {
        let next_step = self.next_guard_pos();

        self.steps.get(&(self.guard_pos, next_step)).is_some()
    }

    fn run_simulation(&mut self) -> SimulationResult {
        loop {
            if self.no_more_steps() {
                break;
            }
            self.step();
            
            if self.is_guard_in_loop() {
                return SimulationResult::Loop(self.distinct_visited() as i32);
            }
        }

        SimulationResult::Finished(self.distinct_visited() as i32)
    }

    #[allow(unused)]
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
                } else if self.visited.get(&self.pos2point(pos)).is_some() {
                    'X'
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
    let mut sim = GuardSimulator::from_string(input);

    let visited_fields = match sim.run_simulation() {
        SimulationResult::Finished(v) => {v}
        SimulationResult::Loop(v) => {v}
    };

   visited_fields.to_string()
}

pub fn process_part2(input: &str) -> String {
    let sim = GuardSimulator::from_string(input);
    let mut possible_loops = 0;

    for i in 0..sim.board.len()-1 {
        let mut sim_cpy = sim.clone();
        if sim_cpy.is_guard_at(i) || sim_cpy.is_obstruction_at(i) {
            continue
        }

        sim_cpy.put_obstruction(i);

        match sim_cpy.run_simulation() {
            SimulationResult::Finished(_) => {}
            SimulationResult::Loop(_) => {possible_loops += 1}
        }
    }

    format!("{}", possible_loops)
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

