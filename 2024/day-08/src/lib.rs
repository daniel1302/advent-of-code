use std::{collections::{HashMap, HashSet}, convert::From};

use itertools::Itertools;

type Point = (i32, i32);

#[derive(Debug)]
struct FrequencyMap {
    board: Vec<char>,
    board_width: usize,

    frequencies: HashMap<char, Vec<Point>>,
    antinodes: HashSet<Point>,
}

impl FrequencyMap {
    fn from_string<T>(input: T) -> FrequencyMap 
        where T: Into<String> {

        let input: String = input.into();

        let board = input
            .lines()
            .map(|row| row.chars().collect::<Vec<char>>())
            .flatten()
            .collect::<Vec<char>>();
    
    
        let width = input.lines().next().unwrap().len();
    
        FrequencyMap{
            board: board,
            board_width: width,
            frequencies: HashMap::new(),
            antinodes: HashSet::new(),
        }
    }

    fn point2pos(&self, p: Point) -> usize {
        p.0 as usize + p.1 as usize *self.board_width
    }

    fn pos2point(&self, pos: usize) -> Point {
        let y = pos / self.board_width;
        let x = pos % self.board_width;

        (x as i32, y as i32)
    }    

    fn compute_frequencies(&mut self) {
        self.frequencies = self
            .board
            .iter()
            .enumerate()
            .fold(HashMap::new(), 
            |mut acc, (pos, freq)| {
                if *freq != '.' {
                    acc
                        .entry(*freq)
                        .and_modify(|positions| positions.push(self.pos2point(pos)))
                        .or_insert(vec![self.pos2point(pos)]);
                }

                acc
            });
    }

    fn is_point_in_boundaries(&self, p: Point) -> bool {
        p.0 >= 0 
            && p.1 >= 0
            && p.0 < self.board_width as i32
            && p.1 < self.board_width as i32
    }

    fn compute_antinodes(&mut self) {
        self.antinodes = self
            .frequencies
            .iter()
            .fold(HashSet::new(), 
                |acc: HashSet<Point>, (_, positions)| {
                positions
                    .iter()
                    .cartesian_product(positions.iter())
                    .filter(|&(&p1, &p2)| p1 != p2)
                    .map(|(p1, p2)| {
                        (p1.0 + p1.0 - p2.0, p1.1 + p1.1 - p2.1)
                    })
                    .filter(|&p| self.is_point_in_boundaries(p))
                    .merge(acc)
                    .collect()
            });
    }
    
    fn compute_resonant_harmonics_antinodes(&mut self) {
        self.antinodes = self
            .frequencies
            .iter()
            .fold(HashSet::new(), 
                |acc: HashSet<Point>, (_, positions)| {
                positions
                    .iter()
                    .cartesian_product(positions.iter())
                    .filter(|&(&p1, &p2)| p1 != p2)
                    .map(|(p1, p2)| {
                        let vec2 = (p1.0-p2.0, p1.1-p2.1);
                        let mut res = vec![
                            p1.clone(),
                            p2.clone(),
                        ];

                        let mut point = (p1.0 + vec2.0, p1.1 + vec2.1);
                        while self.is_point_in_boundaries(point) {
                            res.push(point.clone());

                            point = (point.0 + vec2.0, point.1 + vec2.1);
                        }

                        res
                    })
                    .flatten()
                    .merge(acc)
                    .collect()
            });
    } 

    fn count_unique_antinodes(&self) -> usize {
        self
            .antinodes
            .iter()
            .count()
    }
}

impl From<&str> for FrequencyMap {
    fn from(input: &str) -> Self {
        Self::from_string(input)
    }
}

impl std::fmt::Display for FrequencyMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res: String = self
            .board
            .iter()
            .enumerate()
            .fold(String::new(), |acc, (pos, c)| {
                let mut acc = acc.clone();

                if pos % self.board_width == 0 {
                    acc.push('\n');
                }

                match c {
                    '.' => {
                        acc.push(
                            self
                                .antinodes
                                .get(&self.pos2point(pos))
                                .map_or('.', |_| '#')
                        );
                    },
                    _ => acc.push(*c),
                }

                acc
            })
            ;

        write!(f, "{}", res)
    }
}

pub fn process_part1(input: &str) -> String {
    let mut map: FrequencyMap = input.into();
    map.compute_frequencies();
    map.compute_antinodes();
    format!("{}", map.count_unique_antinodes())
}

pub fn process_part2(input: &str) -> String {
    let mut map: FrequencyMap = input.into();
    map.compute_frequencies();
    map.compute_resonant_harmonics_antinodes();

    format!("{}", map.count_unique_antinodes())
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    const INPUT2: &str = "T.........
...T......
.T........
..........
..........
..........
..........
..........
..........
..........";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT2));
        println!("{}", process_part2(INPUT));
    }
}