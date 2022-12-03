
const POINTS_ALPHABET: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn find_comon_in_groups(groups: &[String]) -> Option<char> {
    for letter in POINTS_ALPHABET.chars() {
        let mut common = true;
        for group in groups {
            if group.find(letter).is_none() {
                common = false;
                break;
            }
        }

        if common {
            return Some(letter);
        }
    }

    return None
}

pub fn process_part1(input: &str) -> String {
    input.lines().map(|line| {
        let line_chunks = line.chars()
            .collect::<Vec<_>>()
            .chunks(line.len()/2)
            .map(|c| c.iter().collect::<String>())
            .collect::<Vec<_>>();

        match find_comon_in_groups(&line_chunks) {
            Some(common_item) => POINTS_ALPHABET.to_string().find(common_item).unwrap_or(0) as u32,
            _ => 0,
        }
    })
    .sum::<u32>().to_string()
}

pub fn process_part2(input: &str) -> String {
    input.lines()
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .chunks(3)
        .map(|line| {
        match find_comon_in_groups(&line) {
            Some(common_item) => POINTS_ALPHABET.to_string().find(common_item).unwrap_or(0) as u32,
            _ => 0,
        }
    })
    .sum::<u32>().to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

    #[test]
    fn part1() {
        assert_eq!(process_part1(INPUT), "157");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(INPUT), "70");
    }
}
