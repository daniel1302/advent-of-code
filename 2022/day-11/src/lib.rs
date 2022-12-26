use std::{collections::VecDeque, fmt::Display, cmp::Ordering};

use nom::{
    IResult,
    character::{
        complete::{digit1, multispace0, line_ending, alpha1, alpha0}, 
    },
    sequence::{preceded, terminated, tuple},
    bytes::complete::{tag, take_till, take_while}, 
    multi::separated_list0, 
    combinator::opt, 
    branch::alt, 
    error::Error,
};


fn parse_monkey(input: &str) -> IResult<&str, &str> {
        terminated(
        preceded(
            tuple((multispace0, tag("Monkey"), multispace0)), 
            digit1
        ),
        tuple((tag(":"), opt(line_ending))),
    )(input)
}

fn parse_starting_items(input: &str) -> IResult<&str,Vec<&str>> {
    terminated(
    preceded(
        take_till(|c: char| c.is_digit(10)), 
        separated_list0(
            tuple((tag(","), opt(multispace0))),
            digit1,
        )
    ),
    opt(line_ending),
    )(input)
}

fn parse_operation(input: &str) -> IResult<&str, Operation> {
    let (input, (operand1, operator, operand2)) = terminated(
        preceded(
            tuple((multispace0, tag("Operation: new ="), multispace0)),
            tuple((
                alt((tag("old"), digit1)),
                preceded(
                    opt(multispace0), 
                    alt((tag("+"), tag("*"))),
                ),
                preceded(
                    opt(multispace0), 
                    alt((digit1, tag("old")))
                ),
            )),
        ),
        opt(line_ending),
    )(input)?;

    let o1 = match operand1 {
        "old" => Operand::Old,
        n => Operand::Num(n.parse().unwrap_or(0)),
    };
    let o2 = match operand2 {
        "old" => Operand::Old,
        n => Operand::Num(n.parse().unwrap_or(0)),
    };

    return match operator {
        "+" => Ok((input, Operation::Add(o1, o2))),
        "*" => Ok((input, Operation::Mul(o1, o2))),
        _ => Err(nom::Err::Incomplete(nom::Needed::Unknown)),
    }
}

fn parse_test(input: &str) -> IResult<&str, (&str, &str)> {
    terminated(
        preceded(
            tuple((multispace0, tag("Test:"), multispace0)),
            tuple((
                alpha1, 
                preceded(tuple((opt(multispace0), opt(alpha0), opt(multispace0))), digit1),
            )),
        ),
        opt(line_ending),
    )(input)
}

fn parse_test_branch(input: &str) -> IResult<&str, &str> {
    terminated(
        preceded(
            tuple((multispace0, tag("If "), alt((tag("true"), tag("false"))), tag(":"), multispace0, tag("throw to monkey"), multispace0)),
            digit1
        ),
        opt(line_ending),
    )(input)
}

#[derive(Debug, Clone)]
enum Operand {
    Old,
    Num(i32),
}

impl PartialEq for Operand {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Num(t0), Self::Num(r0)) => t0 == r0,
            _ => core::mem::discriminant(self) == core::mem::discriminant(other),
        }
    }
}

#[derive(Debug, Clone)]
enum Operation {
    Add(Operand, Operand),
    Mul(Operand, Operand),
}

impl PartialEq for Operation {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Add(t1, t2), Self::Add(r0, r1)) => t1 == r0 && t2 == r1,
            (Self::Mul(t1, t2), Self::Mul(r0, r1)) => t1 == r0 && t2 == r1,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
struct MonekyBehaviour {
    id: u32,
    items: VecDeque<u64>,
    operation: Operation,
    test: i32,
    true_receiver: u64,
    false_receiver: u64,
    inspected: u64,
}

fn print_monkeys(monkeys: &Vec<MonekyBehaviour>) {
    for monkey in monkeys.iter() {
        println!("Monkey {}: {:?}, inspected {:}", monkey.id, monkey.items, monkey.inspected)
    }
} 

impl PartialEq for MonekyBehaviour {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && 
            self.items == other.items && 
            self.operation == other.operation && 
            self.test == other.test && 
            self.true_receiver == other.true_receiver && 
            self.false_receiver == other.false_receiver
    }
}

fn parse_single(input: &str) -> IResult<&str, MonekyBehaviour> {
    let (input, _) = take_while::<_, _, Error<_>>(|c: char| !c.is_alphanumeric())(input)?;
    let (input, monkey_id) = parse_monkey(input)?;
    let (input, starting_items) = parse_starting_items(input)?;
    let (input, operation) = parse_operation(input)?;
    let (input, (_, test_operand)) = parse_test(input)?;
    let (input, throw_to_when_true) = parse_test_branch(input)?;
    let (input, throw_to_when_false) = parse_test_branch(input)?;

    Ok((input, MonekyBehaviour { 
        id: monkey_id.parse().unwrap(), 
        items: starting_items.iter().map(|item| item.parse().unwrap()).collect(), 
        operation: operation,
        test: test_operand.parse().unwrap_or(0), 
        true_receiver: throw_to_when_true.parse().unwrap_or(0), 
        false_receiver: throw_to_when_false.parse().unwrap_or(0),
        inspected: 0,
    }))
} 

fn parse_input(input: &str) -> IResult<&str, Vec<MonekyBehaviour>> {
    let mut input = input;
    let mut result: Vec<MonekyBehaviour> = vec![];
    while let Ok((output, monkey)) = parse_single(input) {
        result.push(monkey);
        input = output;
    }

    return Ok((input, result));
}

pub fn process_part1(input: &str) -> String {
    let (_, mut monkeys) = parse_input(input).unwrap();

    for round in 1..21 {
        for i in 0..monkeys.len() {
            let current_monkey = monkeys[i].clone();

            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspected+=1;
                // println!("Monkey {} inspect element {}", current_monkey.id, item);
                let worry_level = match monkeys[i].operation {
                    Operation::Add(Operand::Old, Operand::Num(interest)) |
                    Operation::Add(Operand::Num(interest), Operand::Old) => item + interest as u64,
                    Operation::Add(Operand::Old, Operand::Old) => item + item,
                    Operation::Mul(Operand::Old, Operand::Num(interest)) |
                    Operation::Mul(Operand::Num(interest), Operand::Old) => item * interest as u64,
                    Operation::Mul(Operand::Old, Operand::Old) => item * item,
                    _ => 0,
                } / 3;
                
                if worry_level % current_monkey.test as u64 == 0 {
                    monkeys[current_monkey.true_receiver as usize].items.push_back(worry_level);
                } else {
                    monkeys[current_monkey.false_receiver as usize].items.push_back(worry_level);
                }
            }
        }

        // println!("\nRound {round:}");
        // print_monkeys(&monkeys);
    }

    let mut inspected = monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<u64>>();
    inspected.sort_by(|v1, v2| {
        if v1 == v2 { Ordering::Equal }
        else if v1 > v2 { Ordering::Less }
        else { Ordering::Greater } 
    });

    return (inspected[0] * inspected[1]).to_string()
}

pub fn process_part2(input: &str) -> String {
    let (_, mut monkeys) = parse_input(input).unwrap();

    let mod_num = monkeys.iter().map(|monkey| monkey.test as u64).product::<u64>();
    for _ in 1..10_001 {
        for i in 0..monkeys.len() {
            let current_monkey = monkeys[i].clone();

            while let Some(item) = monkeys[i].items.pop_front() {
                monkeys[i].inspected+=1;
                let worry_level = match monkeys[i].operation {
                    Operation::Add(Operand::Old, Operand::Num(interest)) |
                    Operation::Add(Operand::Num(interest), Operand::Old) => item%mod_num + interest as u64,
                    Operation::Add(Operand::Old, Operand::Old) => item%mod_num + item%mod_num,
                    Operation::Mul(Operand::Old, Operand::Num(interest)) |
                    Operation::Mul(Operand::Num(interest), Operand::Old) => item%mod_num * interest as u64,
                    Operation::Mul(Operand::Old, Operand::Old) => item%mod_num * item%mod_num,
                    _ => 0,
                };
                
                if worry_level % current_monkey.test as u64 == 0 {
                    monkeys[current_monkey.true_receiver as usize].items.push_back(worry_level);
                } else {
                    monkeys[current_monkey.false_receiver as usize].items.push_back(worry_level);
                }
            }
        }
    }

    let mut inspected = monkeys.iter().map(|monkey| monkey.inspected).collect::<Vec<u64>>();
    inspected.sort_by(|v1, v2| {
        if v1 == v2 { Ordering::Equal }
        else if v1 > v2 { Ordering::Less }
        else { Ordering::Greater } 
    });

    return (inspected[0] * inspected[1]).to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_SINGLE: &str = "Monkey 0:
Starting items: 79, 98
Operation: new = old * 19
Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3";

    const INPUT: &str = "Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1";

    #[test]
    fn part2() {
        assert_eq!("2713310158", process_part2(INPUT));
    }

    #[test]
    fn part1() {
        assert_eq!("10605", process_part1(INPUT));
    }

    #[test]
    fn parse_all(){
        let (_, monkeys) = parse_input(INPUT).unwrap();
        assert_eq!(4, monkeys.len());
    }

    #[test]
    fn parse_sinble_into_struct() {
        assert_eq!(("", MonekyBehaviour{
            id: 0,
            items: [79, 98].into(),
            operation: Operation::Mul(Operand::Old, Operand::Num(19)),
            test: 23,
            true_receiver: 2,
            false_receiver: 3,
            inspected: 0,
        }), parse_single(INPUT_SINGLE).unwrap());
    }

    #[test]
    fn parse_single_separated() {
        let (input, monkey_id) = parse_monkey(INPUT_SINGLE).unwrap();
        assert_eq!(input, "Starting items: 79, 98\nOperation: new = old * 19\nTest: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3");
        assert_eq!(monkey_id, "0");

        let (input, starting_items) = parse_starting_items(input).unwrap();
        assert_eq!(input, "Operation: new = old * 19\nTest: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3");
        assert_eq!(starting_items, vec!["79", "98"]);

        let (input,operation) = parse_operation(input).unwrap();
        assert_eq!(input, "Test: divisible by 23\n    If true: throw to monkey 2\n    If false: throw to monkey 3");
        assert_eq!(operation, Operation::Mul(Operand::Old, Operand::Num(19)));

        let (input, (operator, operand)) = parse_test(input).unwrap();
        assert_eq!(input, "    If true: throw to monkey 2\n    If false: throw to monkey 3");
        assert_eq!(operator, "divisible");
        assert_eq!(operand, "23");

        let (input, throw_to) = parse_test_branch(input).unwrap();
        assert_eq!(input, "    If false: throw to monkey 3");
        assert_eq!(throw_to, "2");

        
        let (input, throw_to) = parse_test_branch(input).unwrap();
        assert_eq!(input, "");
        assert_eq!(throw_to, "3");
    }
}