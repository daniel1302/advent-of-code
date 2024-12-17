use nom::{
    bytes::{
        complete::tag, 
        streaming::is_a
    }, 
    character::complete::{digit1, newline}, 
    combinator::{map, opt}, 
    multi::{fold_many1, many1}, 
    sequence::{preceded, terminated, tuple}, 
    IResult
};

#[derive(Debug, Clone)]
struct Computer {
    register_a: i128,
    register_b: i128,
    register_c: i128,

    program: Vec<u8>,
    pc: usize,

    output: Vec<u8>,

    a_multiplier: i128
}

impl Computer {
    fn xdv(&self, op: u8) -> i128 {
        self.register_a / 2i128.pow(self.combo_op(op) as u32)
    }

    fn adv(&mut self, op: u8) { // 0
        self.a_multiplier = self.a_multiplier.max(2i128.pow(self.combo_op(op) as u32));

        self.register_a = self.xdv(op);
        

        self.pc += 2;
    }

    fn bxl(&mut self, op: u8) { // 1
        self.register_b = self.register_b ^ op as i128;
        self.pc += 2;
    }

    fn bst(&mut self, op: u8) { // 2
        self.register_b = self.combo_op(op).rem_euclid(8);
        self.pc += 2;
    }

    fn jnz(&mut self, op: u8) { // 3
        if self.register_a == 0 {
            self.pc += 2;
        } else {
            self.pc = op as usize
        }
    }

    fn bxc(&mut self, _op: u8) { // 4
        self.register_b = self.register_b ^ self.register_c;
        self.pc += 2;
    }

    fn out(&mut self, op: u8) { // 5
        self.output.push((self.combo_op(op).rem_euclid(8)) as u8);
        self.pc += 2;
    }

    fn bdv(&mut self, op: u8) {  // 6
        self.register_b = self.xdv(op);
        self.pc += 2;
    }
    fn cdv(&mut self, op: u8) { // 7
        self.register_c = self.xdv(op);      
        self.pc += 2;  
    }

    fn combo_op(&self, op: u8) -> i128 {
        match op {
            1|2|3 => op as i128,
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => panic!("Invalid operand")
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.is_halted() {
                break
            }
    
            self.tick();    
        }
    }

    pub fn tick(&mut self) {
        let instruction = self.program.get(self.pc).unwrap();
        let op = self.program.get(self.pc+1).unwrap().to_owned();

        match instruction {
            0 => self.adv(op),
            1 => self.bxl(op),
            2 => self.bst(op),
            3 => self.jnz(op),
            4 => self.bxc(op),
            5 => self.out(op),
            6 => self.bdv(op),
            7 => self.cdv(op),
            _ => panic!("Invalid op")
        }
    }

    pub fn is_halted(&self) -> bool {
        self.pc+1 >= self.program.len() // +1 because of opcode
    }

    pub fn get_output(&self) -> String {
        self.output
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn get_program(&self) -> String {
        self.program
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    pub fn set_register_a(&mut self, val: i128) {
        self.register_a = val;
    }

    pub fn reset(&mut self) {
        self.a_multiplier = 1;
        self.register_b = 0;
        self.register_a = 0;
        self.register_c = 0;
        self.pc = 0;
        self.output = vec![];
    }

}

fn computer_parser(input: &str) -> IResult<&str, Computer> {
    map(
        tuple((
                terminated(
                    preceded(
                        tag("Register A: "), 
                        digit1,
                    ),
                    newline,
                ),

                terminated(
                    preceded(
                        tag("Register B: "), 
                        digit1,
                    ),
                    newline,
                ),

                terminated(
                    preceded(
                        tag("Register C: "), 
                        digit1,
                    ),
                    newline,
                ),

                preceded(
                    tuple((many1(is_a(" \n")), tag("Program: "))), 
                    fold_many1(
                        terminated(digit1, opt(tag(","))),
                        Vec::new,
                        |mut acc: Vec<u8>, item: &str| {
                            acc.push(item.parse().unwrap());
                            acc
                        }
                    ),
                )
        )),

        |(reg_a, reg_b, reg_c, prog): (&str, &str, &str, Vec<u8>)| {
            Computer{
                register_a: reg_a.parse::<i128>().unwrap(),
                register_b: reg_b.parse::<i128>().unwrap(),
                register_c: reg_c.parse::<i128>().unwrap(),

                program: prog,
                pc: 0,
                output: vec![],

                a_multiplier: 1,
            }
        }
    )(input)
}

fn parse_input(input: &str) -> Computer {
    let (_, computer) = computer_parser(input).unwrap();

    computer
}

pub fn process_part1(input: &str) -> String {
    let mut computer = parse_input(input);
    computer.run();    
    computer.get_output()
}

pub fn process_part2(input: &str) -> String {
    let mut computer = parse_input(input);

    let is_valid_output = |program: &Vec<u8>, output: &Vec<u8>| {
        if output.len() > program.len() {
            return false
        }

        output[..]
            .iter()
            .zip(program[program.len()-output.len()..].iter())
            .all(|(&out, &prog)| {
                out == prog
            })
    };

    let mut new_register_a: i128 = 1;
    loop {
        computer.reset();
        computer.set_register_a(new_register_a);
        computer.run();      

        if is_valid_output(&computer.program, &computer.output) {
            if computer.get_output() == computer.get_program() {
                break;
            }
            new_register_a = new_register_a*computer.a_multiplier;
        } else {
            new_register_a+=1;
        }
    }

    
    new_register_a.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const INPUT2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT2));
    }
}