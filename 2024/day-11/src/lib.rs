use rayon::prelude::*;

fn parse_input(input: &str) -> Vec<u128> {
    input
        .trim()
        .split_whitespace()
        .map(|stone| stone.parse::<u128>().unwrap())
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
        for i in 0..stone_digits/2 {
            higher_half = higher_half / 10;
        }

        Some(vec![
            higher_half,
            val-(higher_half*(10 as u128).pow(stone_digits as u32/2))
        ])
    }
}

fn apply_rule(stone_val: u128) -> Vec<u128> {
    // If the stone is engraved with the number 0, it is replaced 
    // by a stone engraved with the number 1.
    if stone_val == 0 {
        return vec![1];
    }

    // If the stone is engraved with a number that has an even number 
    // of digits, it is replaced by two stones. The left half of the 
    // digits are engraved on the new left stone, and the right half of 
    // the digits are engraved on the new right stone. (The new numbers 
    // don't keep extra leading zeroes: 1000 would become stones 10 
    // and 0.)
    if let Some(two_stones) = split_stone(stone_val) {
        return two_stones;
    }

    vec![stone_val*2024]
}

fn blink(initial_stones: &[u128], blinks: usize) -> Vec<u128> {
    (0..blinks).fold(Vec::from(initial_stones), |acc, _| {
        let res = acc
            .iter()
            .map(|stone| apply_rule(*stone))
            .flatten()
            .collect();
        res
    })
}

pub fn process_part1(input: &str) -> String {
    let initial_stones = parse_input(input);
    blink(&initial_stones, 35)
        .len()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let initial_stones = parse_input(input);

    blink(&initial_stones, 35)
        .chunks(5)
        .enumerate()
        .par_bridge()
        .map(|(i, items_chunk)| {
            println!("Chunk {}/12309881", i);
            blink(items_chunk, 35)
                .len()
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