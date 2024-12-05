use regex::Regex;

#[derive(Debug)]
enum Op {
    Unknown,
    DoNot,
    Do,
    Mul(i32, i32),
}

fn parse_input(input: &str) -> Vec<Op> {
    let re = Regex::new(
        r"(?<mnemonic>mul\((?<p1>\d+),(?<p2>\d+)\)|do\(\)|don't\(\))",
    ).unwrap();

    re.captures_iter(input).map(|caps| {
        let mnemonic = caps.name("mnemonic").unwrap().as_str();
        if mnemonic.starts_with("don't") {
            Op::DoNot
        } else if mnemonic.starts_with("do") {
            Op::Do
        } else if mnemonic.starts_with("mul") {
            caps.name("p1").unwrap().as_str();
            caps.name("p2").unwrap().as_str();

            Op::Mul(
                caps.name("p1").unwrap().as_str().parse::<i32>().unwrap(),
                caps.name("p2").unwrap().as_str().parse::<i32>().unwrap(),
            )
        } else {
            Op::Unknown
        }
    })
    .collect()
}

fn eval(program: Vec<Op>, with_disabled_memory: bool) -> i32 {
    let mut disabled = false;
    let mut sum = 0;
    for op in program {
        match op {
            Op::Mul(a, b) => {
                if !with_disabled_memory || !disabled {
                    sum += a*b;
                }
            }
            Op::Do => disabled = false,
            Op::DoNot => disabled = true,
            Op::Unknown => {},
        }
    }

    sum
}

pub fn process_part1(input: &str) -> String {
    let program = parse_input(input);
    eval(program, false).to_string()
}

pub fn process_part2(input: &str) -> String {
    let program = parse_input(input);
    eval(program, true).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT1: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT1));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT2));
    }
}