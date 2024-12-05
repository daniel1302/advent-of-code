use std::{cmp::Ordering, collections::HashMap, vec};
use itertools::Itertools;

type OrderingRules = HashMap<i32, Vec<i32>>;
type Updates = Vec<Vec<i32>>;

fn parse_input(input: &str) -> (OrderingRules, Updates) {
    let ordering_rules_chunks = input
        .lines()
        .filter_map(
            |val| {
                let rule: Vec<i32> = val
                    .split("|")
                    .filter_map(|v| v.parse::<i32>().ok() )
                    .collect();

                if rule.len() == 2 {
                    Some((rule.get(0).unwrap().to_owned(), rule.get(1).unwrap().to_owned()))
                } else {
                    None
                }                
            }
        );

        let updates: Updates = input
        .lines()
        .filter_map(
            |val| {
                let rule: Vec<i32> = val
                    .split(",")
                    .filter_map(|v| v.parse::<i32>().ok() )
                    .collect();

                if rule.len() > 1 {
                    Some(rule)
                } else {
                    None
                }                
            }
        )
        .collect();

        let ordering_rules: OrderingRules = {
            let mut ordering_rules = HashMap::new();
            for rule in ordering_rules_chunks {
                ordering_rules.entry(rule.0).or_insert(vec![]);
                ordering_rules.get_mut(&rule.0).unwrap().push(rule.1);
            }

            ordering_rules
        };


    (ordering_rules, updates)
}

fn check_update(rules: &OrderingRules, update: &Vec<i32>) -> bool {
    let correct_order = |page, pages: &[i32]| -> bool {
        let empty_rule = vec![];
        let page_rule = rules.get(page).unwrap_or(&empty_rule);
        pages
            .iter()
            .all(|p| page_rule.contains(p))
    };

    let violates_rule = |page: i32, pages: &[i32]| -> bool {

    };

    update
        .iter()
        .enumerate()
        .all(|(idx, cur_page)| {
                correct_order(cur_page, &update[idx+1..])
        })
}

pub fn process_part1(input: &str) -> String {
    let(rules, updates) =  parse_input(input);

    println!("{:?}", rules);
    // let violates_rule: bool = |p1: i32, p2: i32| 

    let valid_updates: Vec<&Vec<i32>> = updates
        .iter()
        .filter(|&update| {
            check_update(&rules, update)
        })
        .collect();

    println!("{:?}", valid_updates);

    "".to_string()
}

pub fn process_part2(input: &str) -> String {
   "".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}
