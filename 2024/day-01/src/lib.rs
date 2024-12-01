
pub fn parse_input(input: &str) -> (Vec<i32>, Vec<i32>) {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();

    for (i, token) in input.split_ascii_whitespace().enumerate() {
        if i % 2 == 0 {
            first_list.push(token.parse().unwrap());
        } else {
            second_list.push(token.parse().unwrap());
        }
    }

    first_list.sort();
    second_list.sort();

    (first_list, second_list)
}

pub fn process_part1(input: &str) -> String {
    let (first_list, second_list) = parse_input(input);

    first_list
        .iter()
        .zip(second_list.iter())
        .fold(0, |sum,(&l, &r)| sum + (l-r).abs() )
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (first_list, second_list) = parse_input(input);

    first_list
        .iter()
        .map(|&left_id| {
            second_list
                .iter()
                .clone()
                .filter(|&&right_id| left_id == right_id)
                .count() * left_id as usize
        })
        .sum::<usize>()
        .to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}