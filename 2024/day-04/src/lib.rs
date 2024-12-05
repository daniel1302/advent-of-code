use std::vec;

use itertools::Itertools;

fn parse_input(input: &str) -> Vec<String> {
    input
        .lines()
        .map(|l| l.to_owned())
        .collect()
}

fn build_possibility_matrix(input: Vec<String>) -> Vec<String> {
    let input_reversed = input
        .iter()
        .map(|x| x.clone().chars().rev().collect::<String>())
        .collect::<Vec<String>>();

    // left to right in line
    let mut result = input.clone();

    // right to left in line
    result.append(&mut input_reversed.clone());


    // Top to bottom and bottom to top
    for x in 0..(input.get(0).unwrap().len()) {
        let mut word = String::new();
        for y in 0..input.len() {
            word.push(input.get(y).unwrap().chars().nth(x).unwrap());
        }

        result.push(word.clone());
        result.push(word.chars().rev().collect::<String>());
    }

    // let mut result: Vec<String> = vec![];

    // let mut result: Vec<String> = vec![];
    let y_max = input.len();
    let x_max= input.get(0).unwrap().len();
    for x in 0..x_max {
        let mut word = String::new();
        let mut word_reversed = String::new();

        let mut letter = 0;
        while letter <= x {
            let r_y = letter;
            let r_x = x - letter;
            if r_x > x_max - 1 || r_y > y_max - 1 {
                
                letter+=1;
                continue
            }

            word.push(input.get(r_y).unwrap().chars().nth(r_x).unwrap());
            word_reversed.push(input_reversed.get(r_y).unwrap().chars().nth(r_x).unwrap());
            letter+=1;
        }
        result.push(word.clone());
        result.push(word.chars().rev().collect::<String>());
        result.push(word_reversed.clone());
        result.push(word_reversed.chars().rev().collect::<String>());
    }

    for y in 1..y_max {
        let mut word = String::new();
        let mut word_reversed = String::new();

        let mut letter = 0;
        while letter <= x_max  {
            let r_y = y+letter;
            if x_max - letter < 1 || r_y > y_max - 1 {
                letter+=1;
                continue
            }
            let r_x = x_max - letter - 1;

            word.push(input.get(r_y).unwrap().chars().nth(r_x).unwrap());
            word_reversed.push(input_reversed.get(r_y).unwrap().chars().nth(r_x).unwrap());
            letter+=1;
        }
        result.push(word.clone());
        result.push(word.chars().rev().collect::<String>());
        result.push(word_reversed.clone());
        result.push(word_reversed.chars().rev().collect::<String>());
    }

    

    print!("{:?}", result);
    result
}

pub fn process_part1(input: &str) -> String {
    let matrix = build_possibility_matrix(parse_input(input));
    
    matrix
        .iter()
        .map(|line| {
            line.match_indices("XMAS").count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let input  = parse_input(input);

    let get_char = |x: usize, y: usize| input.get(y).unwrap().chars().nth(x).unwrap();
    let found_x_mass = |x: usize, y: usize| -> bool {
        let center_char = get_char(x,y) == 'A';

        let left_branch = (get_char(x-1, y-1) == 'M' && get_char(x+1, y+1) == 'S')
            || (get_char(x-1, y-1) == 'S' && get_char(x+1, y+1) == 'M');
        let right_branch = (get_char(x+1, y-1) == 'M' && get_char(x-1, y+1) == 'S')
            || (get_char(x+1, y-1) == 'S' && get_char(x-1, y+1) == 'M');

        center_char && left_branch && right_branch
    };

    (1..(input.get(0).unwrap().len()-1))
        .cartesian_product(1..(input.len()-1))
        .map(|(x, y)| found_x_mass(x,y) as i32 )
        .sum::<i32>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
// "XXSAMX", "XMASXX", "MM..A.S", "S.A..MM", "AS.AS...", "...SA.SA", "S.A.A-SA-", "-AS-A.A.S", ".-.MM.....", ".....MM.-.", ".-SX.SAMX", "XMAS.XS-.", ".-.X.---", "---.X.-.", "XMASAMX", "XMASAMX", "M....-", "-....M", "ASAMX", "XMASA", "S.-M", "M-.S", "AMA", "AMA", "MS", "SM", "X", "X"]5
    const INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";


    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}
