use nom::{
    bytes::complete::tag, 
    character::complete::{self, line_ending, space1}, 
    multi::separated_list1, 
    sequence::separated_pair, IResult
};

struct Calibration {
    result: u128,
    params: Vec<u64>
}

impl Calibration {
    fn is_valid(&self, with_concatenation: bool) -> bool {
        let computation_closures = match with_concatenation {
            false => vec![
                |a, b| a+b, 
                |a,b| a*b,
            ],
            true => vec![
                |a, b| a+b, 
                |a,b| a*b, 
                |a, b| format!("{}{}", a, b).parse::<u64>().unwrap()
            ],
        };

        self
            .params
            .iter()
            .skip(1)
            .fold(vec![self.params[0]], |possible_values, val | {
                possible_values
                    .iter()
                    .fold(Vec::new(), |mut new_values, curr_sum| {
                        new_values.extend(
                            computation_closures
                                .iter()
                                .map(|expr| expr(*curr_sum, val))
                                
                        );

                        new_values
                    })
            })
            .iter()
            .filter(|&&computation_result| computation_result as u128 == self.result)
            .count() > 0
    }
}


fn parse_input(input: &str) -> Vec<Calibration>{
    let parser_result: IResult<&str, Vec<(u128, Vec<u64>)>>  = separated_list1(
        line_ending, 
        separated_pair(
            complete::u128, 
            tag(": "), 
            separated_list1(
                space1, 
                complete::u64
            )
        )
    )(input);

    let (_, equations) = parser_result.unwrap(); 

    equations
        .iter()
        .map(|(res, params)| Calibration{
            result: *res,
            params: params.clone(),
        })
        .collect()
}

pub fn process_part1(input: &str) -> String {
    parse_input(input)    
        .iter()
        .filter(|calibration| calibration.is_valid(false))
        .map(|calibration| calibration.result)
        .sum::<u128>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    parse_input(input)    
        .iter()
        .filter(|calibration| calibration.is_valid(true))
        .map(|calibration| calibration.result)
        .sum::<u128>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}