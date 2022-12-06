use std::collections::HashSet;

fn process_unique_seq(input: &str, seq_len: usize) -> u32 {
    for char_no in 0..(input.len()-seq_len-1) {
        if input.get(char_no..char_no+seq_len).unwrap().chars().collect::<HashSet<char>>().len() == seq_len {
            return (char_no + seq_len) as u32
        }
    }
    return 0
}

pub fn process_part1(input: &str) -> String {
    process_unique_seq(input, 4).to_string()
}

pub fn process_part2(input: &str) -> String {
    process_unique_seq(input, 14).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(process_part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), "5");
        assert_eq!(process_part1("nppdvjthqldpwncqszvftbrmjlhg"), "6");
        assert_eq!(process_part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "10");
        assert_eq!(process_part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "11");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), "19");
        assert_eq!(process_part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), "23");
        assert_eq!(process_part2("nppdvjthqldpwncqszvftbrmjlhg"), "23");
        assert_eq!(process_part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), "29");
        assert_eq!(process_part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), "26");
    }
}
