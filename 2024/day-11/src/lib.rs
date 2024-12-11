use std::collections::HashMap;


fn parse_input(input: &str) -> HashMap<u128, usize> {
    input
        .trim()
        .split_whitespace()
        .map(|stone| (stone.parse::<u128>().unwrap(), 1))
        .collect()
}


fn count_digits(num: u128) -> usize {
    if num == 0 {
        return 1;
    }
    
    let mut num = num;
    let mut c = 0;
    while num != 0 {
        num = num/10;
        c+=1;
    }
    
    return c
}


fn split_stone(val: u128) -> Option<Vec<u128>> {
    let stone_digits = count_digits(val);
    if  stone_digits % 2 != 0 {
        None
    } else {
        let mut higher_half = val;
        for _ in 0..stone_digits/2 {
            higher_half = higher_half / 10;
        }

        Some(vec![
            higher_half,
            val-(higher_half*(10 as u128).pow(stone_digits as u32/2))
        ])
    }
}


pub fn blink(numbers: &HashMap<u128, usize>, blinks: usize) -> HashMap<u128, usize> {
    let mut numbers = numbers.clone();
    (0..blinks).for_each(|_| {
        numbers = numbers
            .iter()
            .fold(HashMap::new(), |mut res: HashMap<u128, usize>, (&stone, count)| {
  
                if stone == 0 {
                    *res.entry(1).or_default() += count;
                } else if let Some(two_stones) = split_stone(stone) {
                    *res.entry(two_stones.get(0).unwrap().to_owned()).or_default() += count;
                    *res.entry(two_stones.get(1).unwrap().to_owned()).or_default() += count;
                } else {
                    *res.entry(stone*2024).or_default() += count;    
                }

                res
            })
    });

    numbers
}

pub fn process_part1(input: &str) -> String {
    let initial_stones = parse_input(input);
    blink(&initial_stones, 25)
        .iter()
        .map(|(_, num)| {
            *num
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let initial_stones = parse_input(input);
    blink(&initial_stones, 75)
        .iter()
        .map(|(_, num)| {
            *num
        })
        .sum::<usize>()
        .to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "125 17";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}