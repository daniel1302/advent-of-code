use std::u64;

use nom::{
    bytes::complete::tag, 
    character::complete::{digit1, newline}, 
    combinator::{opt, map}, 
    multi::many0, 
    sequence::{preceded, terminated, tuple}, 
    IResult
};

#[derive(Debug, Clone)]
struct ClawMachine {
    ax: i128,
    ay: i128,
    bx: i128,
    by: i128,
    px: i128,
    py: i128,
}

impl ClawMachine {
    fn solve(&self, press_limit: u64) -> Option<u64> {
        // ax * A + bx * B = px
        // ay * A + by * B = py

        // division by 0
        if self.ay*self.bx-self.ax*self.by == 0 {
            return None
        }

        let a_frac = (-1*(self.by*self.px-self.bx*self.py), self.ay*self.bx-self.ax*self.by);
        let b_frac = (self.ay*self.px-self.ax*self.py, self.ay*self.bx-self.ax*self.by);

        // Denominator must be 1 otherwise there is not combination to solve it
        if a_frac.0 % a_frac.1 != 0 || b_frac.0 % b_frac.1 != 0{
            return None
        }

        let a_tries = (a_frac.0/a_frac.1) as u64;
        let b_tries = (b_frac.0/b_frac.1) as u64;

        if a_tries > press_limit || b_tries > press_limit {
            return None
        }


        Some(3*a_tries + b_tries)
    }
}

fn claw_machine_parser(input: &str) -> IResult<&str, ClawMachine> {
        map(
            preceded(
                opt(many0(newline)),
                tuple((
                    preceded(tag("Button A: X+"), digit1),
                    terminated(preceded(tag(", Y+"), digit1), newline),
                    preceded(tag("Button B: X+"), digit1),
                    terminated(preceded(tag(", Y+"), digit1), newline),
                    preceded(tag("Prize: X="), digit1),
                    terminated(preceded(tag(", Y="), digit1), opt(newline)),
                ))
        ),
        
        |(ax, ay, bx, by, px, py): (&str, &str, &str, &str, &str, &str)| {
            ClawMachine{
                ax: ax.parse::<i128>().unwrap(),
                ay: ay.parse::<i128>().unwrap(),
                bx: bx.parse::<i128>().unwrap(),
                by: by.parse::<i128>().unwrap(),
                px: px.parse::<i128>().unwrap(),
                py: py.parse::<i128>().unwrap(),
            }
        }
    )(input)
}

fn parse_input(input: &str) -> Vec<ClawMachine> {
    let mut input = input;
    let mut res = vec![];
    while input.len() > 0 {
        match claw_machine_parser(input) {
            Ok((rest, claw_machine)) => {
                input = rest;
                res.push(claw_machine);
            },
            Err(e) => {
                panic!("{:?}", e);
            },
        }
    }

    res
}

pub fn process_part1(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|claw_machine|{
            claw_machine.solve(100u64).unwrap_or(0)   
        })
        .sum::<u64>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|claw_machine|{
            let mut claw_machine = claw_machine.clone();
            claw_machine.px += 10000000000000;
            claw_machine.py += 10000000000000;
            claw_machine.solve(u64::MAX).unwrap_or(0)   
        })
        .sum::<u64>()
        .to_string()
}
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}