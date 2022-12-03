use std::str::FromStr;
use std::cmp::{PartialOrd, Ordering};


#[derive(PartialEq,Clone,Copy)]
enum Move {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl FromStr for Move {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "A" | "X" => Ok(Move::Rock),
            "B" | "Y" => Ok(Move::Paper),
            "C" | "Z" => Ok(Move::Scissors),
            _ => Err("invalid move".to_string())
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == &Move::Scissors && other == &Move::Rock {
            return Some(Ordering::Less)
        } else if self == &Move::Rock && other == &Move::Scissors {
            return Some(Ordering::Greater)
        } 

        Some((*self as u8).cmp(&(*other as u8)))
    }

    fn lt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less))
    }

    fn le(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Less | Ordering::Equal))
    }

    fn gt(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater))
    }

    fn ge(&self, other: &Self) -> bool {
        matches!(self.partial_cmp(other), Some(Ordering::Greater | Ordering::Equal))
    }
}

pub fn process_part1(input: &str) -> String {
    input.lines().map(|line| {
        let curr_moves: Vec<Move> = line.split(" ").map(|the_move| the_move.parse::<Move>().unwrap()).collect();
        
        let mut round_points = 0u32;
        if curr_moves[1] > curr_moves[0] {
            round_points += 6;
        } else if curr_moves[0] == curr_moves[1] {
            round_points += 3;
        }

        match curr_moves[1] {
            Move::Rock => round_points += 1,
            Move::Paper => round_points += 2,
            Move::Scissors => round_points += 3,
        }

        round_points
    })
    .sum::<u32>()
    .to_string()
}


pub fn process_part2(input: &str) -> String {
    input.lines().map(|line| {
        let curr_moves: Vec<Move> = line.split(" ").map(|the_move| the_move.parse::<Move>().unwrap()).collect();

        let mut round_points = 0u32;
        match curr_moves[1] {
            Move::Rock => match curr_moves[0] {
                Move::Rock => round_points += Move::Scissors as u32,
                Move::Paper => round_points += Move::Rock as u32,
                Move::Scissors => round_points += Move::Paper as u32,
            }
            Move::Paper => round_points += 3 + curr_moves[0] as u32,
            Move::Scissors => match curr_moves[0] {
                Move::Rock => round_points += 6 + Move::Paper as u32,
                Move::Paper => round_points += 6 + Move::Scissors as u32,
                Move::Scissors => round_points += 6 + Move::Rock as u32,
            },
        }


        round_points
    })
    .sum::<u32>()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z";

    #[test]
    fn part1() {
        assert_eq!(process_part1(INPUT), "15");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(INPUT), "12");
    }
}
