use std::collections::VecDeque;

fn parse_input(input: &str) -> Vec<String> {
    input
        .trim()
        .chars()
        .map(|c| {
            c.to_digit(10).unwrap()
        })
        .collect::<Vec<u32>>()
        .chunks(2)
        .enumerate()
        .map(|(idx, block_info)| {
            match block_info.len() {
                0 => vec![],
                1 => vec![
                        vec![idx.to_string(); block_info[0] as usize]
                    ],
                _ => vec![
                        vec![idx.to_string(); block_info[0] as usize],
                        vec!['.'.to_string(); block_info[1] as usize]
                    ]
            }
        })
        .flatten()
        .flatten()
        .collect()
}

pub fn process_part1(input: &str) -> String {
    let disk = parse_input(input);
    let mut files_on_disk: VecDeque<&str> = disk
        .iter()
        .filter(|block| *block != ".")
        .map(|v| v.as_str())
        .collect();

    println!("Organizing");
    let (mut changed, organized_disk) = disk
        .iter()
        .fold((0, vec![]), |(changed, acc), v| {
            let mut acc = acc.clone();
            let mut changed = changed;
            
            if *v == String::from(".") {
                    if files_on_disk.len() > 0 {
                        acc.push(files_on_disk.pop_back().unwrap().to_owned());
                        changed += 1;
                    } else {
                        acc.push(String::from("."));
                    }
            } else {
                acc.push(v.clone());    
            }

            (changed, acc)
        });
    

    println!("Organizing 2");
    let organized_disk = {
        let mut organized_disk: Vec<String> = organized_disk
        .iter()
        .rev()
        .map(|c| {
            if changed > 0 && *c != String::from(".") {
                changed -= 1;
                ".".to_owned()
            } else {
                c.to_owned()
            }
        })
        .collect();
        organized_disk.reverse();

        organized_disk
    };
    
    
    println!("Computing sum");
    organized_disk
        .iter()
        .enumerate()
        .map(|(idx, block)| {
            println!("IDX: {}, num: {}/{}", idx, block, block.parse::<i32>().unwrap_or(0) as usize);
            
            (idx * block.parse::<i32>().unwrap_or(0)  as usize) as u128
        })
        .sum::<u128>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    "".into()
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2333133121414131402";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}