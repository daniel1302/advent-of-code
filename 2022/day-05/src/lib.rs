use std::vec::Vec;
use std::collections::VecDeque;

fn parse_crates(input: &str) -> Vec<VecDeque<char>> {
    let mut crates_stacks: Vec<VecDeque<char>> = vec![];
    
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    let crates_section = input_parts[0];

    crates_section.lines()
        .for_each(|line| {
            let lines_char_iter = line.chars();
            if lines_char_iter.clone().count() < 1 {
                return;
            }
            
            lines_char_iter.skip(1)
                .step_by(4)
                .enumerate()
                .for_each(|(crate_no, crate_id)| {
                    if crates_stacks.len() <= crate_no {
                        crates_stacks.push(VecDeque::new());
                    }

                    if crate_id >= 'A' && crate_id <= 'Z' {
                        crates_stacks[crate_no].push_back(crate_id);
                    }
                })
        });
   
    return crates_stacks
}

#[derive(Debug)]
struct Move {
    amount: u32,
    from: usize,
    to: usize,
}

fn parse_moves(input: &str) -> Vec<Move> {
    let input_parts = input.split("\n\n").collect::<Vec<&str>>();
    input_parts[1].lines()
        .map(|line| {
            let parts = line.split(" from ").map(|x| x.to_string()).collect::<Vec<String>>();
            let parts_from_to = parts[1].split(" to ").collect::<Vec<&str>>();

            Move{
                amount: (&parts[0][5..]).parse::<u32>().unwrap(),
                from: parts_from_to[0].parse::<usize>().unwrap(),
                to: parts_from_to[1].parse::<usize>().unwrap(),
            }
        })
        .collect::<Vec<Move>>()
}


pub fn process_part1(input: &str) -> String {
    let mut crates_stack = parse_crates(input);

    parse_moves(input).iter().for_each(|item| {
        (0..item.amount).for_each(|_| {
            if let Some(crate_id) = crates_stack[item.from-1].pop_front() {
                crates_stack[item.to-1].push_front(crate_id)
            }
        })
    });

    crates_stack.iter().map(|crates| crates[0]).collect::<String>()
}

pub fn process_part2(input: &str) -> String {
    let mut crates_stack = parse_crates(input);

    parse_moves(input).iter().for_each(|item| {
        crates_stack[item.from-1].rotate_left(item.amount as usize);
        (0..item.amount).rev().for_each(|_| {
            if let Some(crate_id) = crates_stack[item.from-1].pop_back() {
                crates_stack[item.to-1].push_front(crate_id);
            }
        })
    });

    crates_stack.iter().map(|crates| crates[0]).collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2";

    #[test]
    fn part1() {
        assert_eq!(process_part1(INPUT), "CMZ");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(INPUT), "MCD");
    }
}
