fn process_input(input: &str) -> Vec<Vec<i32>> {
    input
        .split("\n")
        .map(|line| line
            .split(" ")
            .map(|val| val.parse::<i32>().unwrap())
            .collect()
        )
        .collect()
}

fn is_report_safe(report_line: &[i32]) -> bool {
    let vals = report_line.iter();
    let next_vals = report_line.iter().skip(1);

    let report_diffs = vals
        .zip(next_vals)
        .map(|(cur, next)| next - cur);

    let all_decreasing = report_diffs
        .clone()
        .all(|item| item < 0);
    
    let all_increasing = report_diffs
        .clone()
        .all(|item| item > 0);

    let all_safe_values = report_diffs
        .clone()
        .all(|item| item.abs() > 0 && item.abs() < 4);

    // println!("Diffs: {:?}", report_diffs.collect::<Vec<i32>>());

    // println!("All safe: {:?}", all_safe_values);
    // println!("All increasing: {:?}", all_increasing);
    // println!("All decreasing: {:?}", all_decreasing);


    return all_safe_values && (all_decreasing || all_increasing)
}

fn is_single_tolerance_safe(report_line: &Vec<i32>) -> bool {
    for i in 0..(report_line.len()) {
        let iter = report_line.iter().clone();

        if is_report_safe( &iter.clone().take(i).chain(iter.skip(i+1)).map(|&v| v).collect::<Vec<i32>>()) {
            return true
        }
    }

    false
}

pub fn process_part1(input: &str) -> String {
    process_input(input)
        .iter()
        .filter(|&report_line| is_report_safe(report_line))
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    process_input(input)
        .iter()
        .filter(|&report_line| is_report_safe(report_line) || is_single_tolerance_safe(report_line))
        .count()
        .to_string()    
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}