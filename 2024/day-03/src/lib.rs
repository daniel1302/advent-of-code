use nom::{
    bytes::complete::tag, number::complete::i32, sequence::{delimited, preceded}
};

// #::IResult::complete::tag[derive(Debug)]
// struct Op {
//     p1: i32,
//     p2: i32
// }

enum Token {
    DoNot,
    Do,
    Mul(i32, i32),
}

// fn parse_mul(input: &str) -> IResult<&str, Token::Mul> {
//     let res = preceded(
//         tag("mul"),
//         delimited(
//             tag("("), 
//             delimited(i32(nom::number::Endianness::Native), ",", i32(nom::number::Endianness::Native)),
//             tag(")")
//         )
//     )(input)?;

//     let 

//     Ok(())
// }

fn process_input(input: &str, with_disabled_memory: bool) -> Vec<Op> {


    // re
    //     .captures_iter(input)
    //     .map(|caps| {
    //         Op{
    //             p1: caps.name("p1").unwrap().as_str().parse::<i32>().unwrap(),
    //             p2: caps.name("p2").unwrap().as_str().parse::<i32>().unwrap(),
    //         }
    //     })
    //     .collect()
}

fn remove_disabled_memory(input: &str) -> String {
    let re = Regex::new(r"don't\(\).+?do\(\)").unwrap();
    re.replace_all(input, "").into_owned()
}

pub fn process_part1(input: &str) -> String {
    let mem = process_input(input);
    println!("{:?}", mem);
    mem
        .iter()
        .fold(0, |sum, op| sum + (op.p1*op.p2))
        .to_string()

}

pub fn process_part2(input: &str) -> String {
    let removed = remove_disabled_memory(input);
    println!("{}", removed);

    let mem = process_input(removed.as_str());
    // println!("{:?}", mem);
    mem
        .iter()
        .fold(0, |sum, op| sum + (op.p1*op.p2))
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const INPUT2: &str = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT2));
    }
}