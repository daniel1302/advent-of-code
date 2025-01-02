use std::{collections::{HashMap, HashSet}, thread::current};

use itertools::Itertools;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{alphanumeric1, digit1, newline},
    combinator::{map, opt},
    multi::fold_many1,
    sequence::{preceded, tuple},
    IResult,
};

type Identifier<'a> = &'a str;

#[derive(Debug)]
enum Expression<'a> {
    And(Identifier<'a>, Identifier<'a>),
    Or(Identifier<'a>, Identifier<'a>),
    Xor(Identifier<'a>, Identifier<'a>),
    Undefined,
}

#[derive(Debug)]
struct Circuit<'a> {
    values: HashMap<&'a str, bool>,
    gates: HashMap<&'a str, Expression<'a>>,
}

fn value_parser(input: &str) -> IResult<&str, (&str, bool)> {
    map(
        preceded(
            opt(newline),
            tuple((
                alphanumeric1,
                preceded(tag(": "), digit1),
            ))
        ),

        |(id, val): (&str, &str)| {
            match val {
                "1" => (id, true),
                _ => (id, false),
            }
        }
    )(input)
}

fn expression_parser(input: &str) -> IResult<&str, (&str, Expression)> {
    use Expression::*;
    map(
        preceded(
            opt(newline),
            tuple((
                alphanumeric1,
                preceded(tag(" "), alt((tag("XOR"), tag("OR"), tag("AND")))),
                preceded(tag(" "), alphanumeric1),
                preceded(tag(" -> "), alphanumeric1),
            )),
        ),
        |(left, op, right, res): (&str,&str,&str,&str)| {
            match op {
                "XOR" => (res, Xor(left, right)),
                "OR" => (res, Or(left, right)),
                "AND" => (res, And(left, right)),
                _ => (res, Undefined),
            }
        }
    )(input)
}

fn empty_parser(input: &str) -> IResult<&str, ()> {
    fold_many1(newline, ||{}, |_, _| {})(input)
}

fn parse_input(input: &str) -> Circuit<'_> {
    let mut values = HashMap::new();
    let mut gates= HashMap::new();

    let mut input = input;
    while !input.is_empty() {
        if let Ok((new_input, (id, expr))) = expression_parser(input) {
            input = new_input;
            gates.insert(id, expr);
        } else if let Ok((new_input, (id, val))) = value_parser(input) {
            input = new_input;
            values.insert(id, val);
        } else if let Ok((new_input, _)) = empty_parser(input) {
            input = new_input;
        } else {
            panic!("Invalid input received: {}", input)
        }
    }

    Circuit { values, gates }
}


impl<'a> Circuit<'a> {
    // can consume Expression if result is produced
    fn compute_expression(&'a self, id: &'a str) -> Option<bool> {
        use Expression::*;
        let expr = self.gates.get(id).unwrap();

        match expr {
            Or(l, r)|Xor(l, r)|And(l, r) => {
                if let (Some(left_val), Some(right_val)) = (self.values.get(r), self.values.get(l)) {
                    return match expr {
                        Or(_, _) => {Some(left_val | right_val) },
                        And(_, _) => {Some(left_val & right_val) },
                        Xor(_, _) => {Some(left_val ^ right_val) },
                        _ => {None},
                    }
                }
            },
            Undefined => panic!("Undefined expression")
        }

        None
    }


    pub fn compute_expressions(&mut self) {
        while !self.gates.is_empty() {
            for item in self.gates.keys().cloned().collect::<Vec<&str>>() {
                if let Some(result) = self.compute_expression(item) {
                    self.gates.remove(item);

                    self.values.insert(item, result);
                }
            }
            
        }
    }

    pub fn result(&self) -> usize {
        let res = self.values
            .iter()
            .filter_map(|(k, _)| {
                if k.starts_with('z') {
                    Some(*k)
                } else {
                    None
                }
            })
            .sorted()
            .map(|id| {
                if *self.values.get(id).unwrap() {
                    '1'
                } else {
                    '0'
                }
            })
            .collect::<String>();

        usize::from_str_radix(&res, 2).unwrap()
    }

    fn print_expression(&self, exp: &str, depth: usize, max_depth: usize) {
        use Expression::*;

        if depth >= max_depth {
            return
        }

        let depth_str = vec!["   "; depth].join("");

        if ['x', 'y'].contains(&exp.chars().nth(0).unwrap()) {
            println!("{}{}", depth_str, exp);
            return;
        }

        let expression = self.gates.get(exp).unwrap();
        match expression {
            Xor(a, b) => {
                println!("{}XOR({})", depth_str, exp);
                self.print_expression(a, depth+1, max_depth);
                self.print_expression(b, depth+1, max_depth);
            },
            And(a, b) => {
                println!("{}AND({})", depth_str, exp);
                self.print_expression(a, depth+1, max_depth);
                self.print_expression(b, depth+1, max_depth);
            },
            Or(a, b) => {
                println!("{}OR({})", depth_str, exp);
                self.print_expression(a, depth+1, max_depth);
                self.print_expression(b, depth+1, max_depth);
            }
            _ => {}
        }
    }
    
    pub fn is_sum_expr(&self, expr: &Expression, output_var: &str) -> bool {
        if let &Expression::Xor(left, right) = expr {
            let bit_no = output_var[1..].parse::<usize>().unwrap();
            let expected_x = format!("x{:02}", bit_no);
            let expected_y = format!("y{:02}", bit_no);
            if left == expected_x {
                left == expected_x && right == expected_y
            } else {
                left == expected_y && right == expected_x
            }
        } else {
            false
        }
    }

    pub fn check_carry_out(&self, expr: &'a Expression, identifier: &'a str) -> Option<&'a str> {
        if let Expression::Or(left, right) = expr {
            let left_expr = self.gates.get(left).unwrap();
            let right_expr = self.gates.get(right).unwrap();

            if !matches!(left_expr, Expression::And(_, _)) {
                return Some(left)
            }  
            
            if !matches!(right_expr, Expression::And(_, _)) {
                return Some(right)
            }
        } else if !matches!(expr, Expression::Or(_, _)) {
            return Some(identifier)
        }
    
        None
    }

    // adder consist of two parts
    //  half adder:
    //      SUM     = A ⊻ B
    //      C_OUT   = A ∧ B
    //  full adder:
    //      SUM     = A ⊻ B ⊻ C_OUT(prev)
    //      C_OUT   = (A ∧ B) ∨ ((A ⊻ B) ∧ C)
    pub fn check_adder_bit_expression(&'a self, expr: &'a str, last_bit: bool) -> Option<&'a str> {
        if !expr.starts_with("z") {
            panic!("invalid expression given to is_sum_equation");
        }

        let z_expr = self.gates.get(expr).unwrap() ;
        
        // the last bit support (no carry out produced)
        if last_bit {
            return if matches!(z_expr, &Expression::Or(_, _)) {
                None
            } else {
                Some(expr)
            }
        }

        if let Expression::Xor(left, right) = z_expr {
            // first bit, is only XOR.
            if expr == "z00" {
                return if (left == &"x00" && right == &"y00") || (left == &"y00" && right == &"x00") {
                    None
                } else {
                    Some(expr)
                }
            }

            let left_expr = self.gates.get(*left).unwrap();
            let right_expr = self.gates.get(*right).unwrap();

            
            if expr == "z01" {
                if self.is_sum_expr(left_expr, expr) && !matches!(right_expr, Expression::And(_, _)) {
                    return Some(right)
                } else if self.is_sum_expr(right_expr, expr) && !matches!(left_expr, Expression::And(_, _)) {
                    return Some(left)
                } 

                return None
            }

            let is_left_sum = self.is_sum_expr(left_expr, expr);
            let is_right_sum = self.is_sum_expr(right_expr, expr);
            let invalid_left_carry_out = self.check_carry_out(left_expr, left);
            let invalid_right_carry_out = self.check_carry_out(right_expr, right);
            
            if is_left_sum && invalid_right_carry_out.is_some() {
                invalid_right_carry_out
            } else if is_right_sum && invalid_left_carry_out.is_some() {
                invalid_left_carry_out
            } else if invalid_left_carry_out.is_none() && !is_right_sum {
                Some(right)
            } else if invalid_right_carry_out.is_none() && !is_left_sum {
                Some(left)
            } else {
                None
            }
        } else {
            Some(expr)
        }
    }

    // pub fn check_adder_sum_equation(expr: &str)
}

pub fn process_part1(input: &str) -> String {
    let mut circuit = parse_input(input);
    circuit.compute_expressions();

    circuit.result().to_string()
}



pub fn process_part2(input: &str) -> String {
    
    let mut circuit = parse_input(input);

    let last_bit = "z45";

    let mut invalid = circuit.gates.iter()
        .filter_map(|(k, _)| {
            if !k.starts_with('z') {
                None
            } else {
               circuit.check_adder_bit_expression(k, *k == last_bit)
            }
        }).collect::<Vec<&str>>();

    invalid.sort();
    invalid.join(",").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

const INPUT2: &str = "x00: 0
x01: 1
x02: 1
x03: 0
x04: 1
y00: 1
y01: 0
y02: 1
y03: 1

x00 AND y00 -> z02
x01 AND y01 -> z01
x02 AND y02 -> z00
x03 AND y03 -> z03";

    const INPUT3: &str = "x00: 0
x01: 1
x02: 0
x03: 1
x04: 0
x05: 1
y00: 0
y01: 0
y02: 1
y03: 1
y04: 0
y05: 1

x00 AND y00 -> z05
x01 AND y01 -> z02
x02 AND y02 -> z01
x03 AND y03 -> z03
x04 AND y04 -> z04
x05 AND y05 -> z00";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT3));
    }
}
