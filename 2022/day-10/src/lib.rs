use std::vec;

#[derive(Debug)]
enum Mnemonic {
    Noop,
    AddX(i32),
}

#[derive(Debug)]
struct CPU {
    cycle: u32,
    reg_x: i32,
    signal_strength: i64,
    crt: Vec<char>,
}

pub fn process_part1(input: &str) -> String {
    let mut cpu = CPU{
        cycle: 0,
        reg_x: 1,
        signal_strength: 0,
        crt: vec![],
    };

    input.lines()
    .map(|line| {
        let instruction = line.split(" ").collect::<Vec<&str>>();

        return match instruction[0] {
            "noop" => vec![Mnemonic::Noop],
            "addx" => vec![Mnemonic::Noop, Mnemonic::AddX(instruction[1].parse::<i32>().unwrap())],
            _ => panic!("Invalid instruction"),
        };
    })
    .flatten()
    .for_each(|mnemonic| {
        cpu.cycle+=1;

        if cpu.cycle == 20 || (20+cpu.cycle)%40 == 0{
            println!("{} * {} = {}", cpu.cycle, cpu.reg_x, cpu.cycle as i64 * cpu.reg_x as i64);
            cpu.signal_strength += cpu.cycle as i64 * cpu.reg_x as i64;
        }

        match mnemonic {
            Mnemonic::Noop => {},
            Mnemonic::AddX(val) => {
                cpu.reg_x+=val;
            } 
        }
    });


    cpu.signal_strength.to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut cpu = CPU{
        cycle: 0,
        reg_x: 1,
        signal_strength: 0,
        crt: vec![],
    };

    input.lines()
    .map(|line| {
        let instruction = line.split(" ").collect::<Vec<&str>>();

        return match instruction[0] {
            "noop" => vec![Mnemonic::Noop],
            "addx" => vec![Mnemonic::Noop, Mnemonic::AddX(instruction[1].parse::<i32>().unwrap())],
            _ => panic!("Invalid instruction"),
        };
    })
    .flatten()
    .for_each(|mnemonic| {
        if [cpu.reg_x-1, cpu.reg_x, cpu.reg_x+1].contains(&((cpu.cycle % 40) as i32)) {
            cpu.crt.push('#');
        } else {
            cpu.crt.push('.')
        }

        cpu.cycle+=1;

        match mnemonic {
            Mnemonic::Noop => {},
            Mnemonic::AddX(val) => {
                cpu.reg_x+=val;
            }
        }
    });
    
    cpu.crt.iter().enumerate()
    .map(|(idx, c)| {
        if idx > 30 && (idx+1)%40 == 0 {
            return format!("{c}\n");
        } else {
            return c.to_string();
        }
    }).collect::<String>()
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";

    #[test]
    fn part1() {
        assert_eq!(process_part1(INPUT), "13140");
    }

    #[test]
    fn part2() {
       assert_eq!(process_part2(INPUT), "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
");
    }
}
