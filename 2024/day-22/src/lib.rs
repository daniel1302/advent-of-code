#![feature(iter_map_windows)]
mod aoclib;
use std::{collections::{BTreeSet, HashMap}};

use rayon::prelude::*;

fn parse_input(input: &str) -> Vec<u128> {
    input
        .trim()
        .lines()
        .map(|l| l.parse::<u128>().unwrap())
        .collect()
}

fn mix(secret: u128, val: u128) -> u128 {
    secret ^ val
} 

fn prune(secret: u128) -> u128 {
    secret.rem_euclid(16777216)
}

fn cur_secret_number(previous: u128) -> u128 {
    let mut new_secret = prune(mix(previous, previous*64));
    new_secret = prune(mix(new_secret, new_secret/32));
    prune(mix(new_secret, new_secret*2048))
}

fn nth_secret(initial_secret: u128, nth: usize) -> u128 {
    let mut res = initial_secret;
    for _ in 0..nth {
        res = cur_secret_number(res);
    }

    res
}


fn prices_diff(initial_secret: u128, nth: usize, registry: &mut HashMap<(i8, i8, i8, i8), i32>) {
    let mut cur_secret = initial_secret;
    let mut seq: (i8, i8, i8, i8) = (0, 0, 0, (initial_secret%10) as i8);
    let mut seen: BTreeSet<(i8, i8, i8, i8)> = BTreeSet::new();
    let mut prev_secret: u128;
    for i in 0..nth {
        prev_secret = cur_secret;
        cur_secret = cur_secret_number(cur_secret);

        let change = (cur_secret%10) as i8 - (prev_secret%10) as i8;

        seq = (seq.1, seq.2, seq.3, change as i8);

        if i < 3 {
            continue
        }

        if seen.contains(&seq) {
            continue
        }
        seen.insert(seq);

        *registry.entry(seq).or_insert(0) += (cur_secret%10) as i32;
    }
}


pub fn process_part1(input: &str) -> String {
    parse_input(input)
        .par_iter()
        .map(|num| nth_secret(*num, 2000))
        .sum::<u128>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {    
    let res = parse_input(input)
        .par_iter()
        .map(|initial_secret|{
            let mut res = HashMap::new();

            prices_diff(*initial_secret, 2000, &mut res);

            res
        })
        .reduce(HashMap::new, |a, b| {
            let mut res = a.clone();
            
            b.iter()
                .for_each(|(k, v)| {
                    *res.entry(*k).or_insert(0) += *v
                });
            
            res
        })
    ;

    

    res.iter().map(|(k,v)| *v).max().unwrap().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "1
10
100
2024";
    const INPUT2: &str = "1
2
3
2024";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT2));
    }
}