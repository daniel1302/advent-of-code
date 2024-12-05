use std::collections::HashMap;

type OrderingRules = HashMap<String, ()>;
type UpdateLine<'a> = Vec<&'a str>;
type Updates<'a> = Vec<UpdateLine<'a>>;

fn parse_input(input: &str) -> (OrderingRules, Updates) {
    let ordering_rules = input
        .lines()
        .filter(|line| line.contains("|"))
        .map(|line| (line.to_owned(), ()))
        .collect::<OrderingRules>();

        let updates: Updates = input
        .lines()
        .filter_map(
            |val| {
                let rule: Vec<&str> = val
                    .split(",")
                    .collect();

                if rule.len() > 1 {
                    Some(rule)
                } else {
                    None
                }                
            }
        )
        .collect();


    (ordering_rules, updates)
}

fn is_valid_update(rules: &OrderingRules, update: &UpdateLine) -> bool {
    update
        .windows(2)
        .all(|pages| rules.contains_key(
            format!("{}|{}", pages.get(0).unwrap(), pages.get(1).unwrap()).as_str()
        ))
}

pub fn process_part1(input: &str) -> String {
    let(rules, updates) =  parse_input(input);

    updates
        .iter()
        .filter(|&update| is_valid_update(&rules, update))
        .map(|update| update
            .get(update.len()/2).unwrap()
            .parse::<i32>()
            .unwrap()
        )
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let(rules, updates) =  parse_input(input);
    let violate_rule = |p1: &str, p2: &str| {
        !rules.contains_key(format!("{}|{}", p1, p2).as_str())
    };

    updates
        .iter()
        .filter(|update| !is_valid_update(&rules, update))
        .map(|update| {
            let mut update_sorted = update.clone();
            update_sorted.sort_by(|&a, &b| {
                match violate_rule(a, b) {
                    true => std::cmp::Ordering::Less,
                    false => std::cmp::Ordering::Greater,
                }
            });

            update_sorted
                .get(update.len()/2).unwrap()
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>()
        .to_string()
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
