use std::collections::HashMap;

use rayon::prelude::*;

type Towels<'a> = Vec<&'a str>;
type Designs<'a> = Vec<&'a str>;

fn parse_input(input: &str) -> (Towels, Designs) {
   let lines = input.lines();

   let towels = lines.clone()
        .take(1)
        .flat_map(|towel| towel.split(", "))
        .collect();

    let designs = lines
        .skip(2)
        .filter(|design| design.len() > 0)
        .collect();

    (towels, designs)
}

fn can_build(design: &str, towels: &Vec<&str>, num: &mut i32) -> bool {
    if design.len() < 1 {
        *num += 1;
        return true;
    }

    for towel in towels {
        if design.starts_with(towel) && can_build(&design[towel.len()..], towels, num) {
            return true;
        }
    }

    return false
}



fn count_possibilities<'a>(
    design: &'a str,
    towels: &Vec<&str>,
    cache: &mut HashMap<&'a str, usize>,
) -> usize {
    if design.len() == 0 {
        return 1;
    }

    if let Some(&count_possibilities) = cache.get(design) {
        return count_possibilities;
    }

    let possibilities = towels
        .iter()
        .filter(|p| design.starts_with(*p))
        .map(|p| count_possibilities(&design[p.len()..], towels, cache))
        .sum();

    cache.insert(design, possibilities);
    possibilities
}

pub fn process_part1(input: &str) -> String {
    let (towels, designs) = parse_input(input);

    designs
        .iter()
        .filter(|design| {
            let mut num = 0;
            can_build(design, &towels, &mut num)
        })
        .count()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let (towels, designs) = parse_input(input);
    designs
        .par_iter()
        .map(|design| {
            let mut cache = HashMap::new();
            
            count_possibilities(design, &towels, &mut cache) as usize
        })
        .sum::<usize>()
        .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}
// The staff don't really like some of the towel arrangements you came up with. To avoid an endless cycle of towel rearrangement, maybe you should just give them every possible option.

// Here are all of the different ways the above example's designs can be made:

// brwrr can be made in two different ways: b, r, wr, r or br, wr, r.

// bggr can only be made with b, g, g, and r.

// gbbr can be made 4 different ways:

//     g, b, b, r
//     g, b, br
//     gb, b, r
//     gb, br

// rrbgbr can be made 6 different ways:

//     r, r, b, g, b, r
//     r, r, b, g, br
//     r, r, b, gb, r
//     r, rb, g, b, r
//     r, rb, g, br
//     r, rb, gb, r

// bwurrg can only be made with bwu, r, r, and g.

// brgr can be made in two different ways: b, r, g, r or br, g, r.

// ubwu and bbrgwb are still impossible.

// Adding up all of the ways the towels in this example could be arranged into the desired designs yields 16 (2 + 1 + 4 + 6 + 1 + 2).

// They'll let you into the onsen as soon as you have the list. What do you get if you add up the number of different ways you could make each design?

fn solve2(design: &str, towels: &Vec<&str>) {
    let mut found_positions: Vec<usize> = vec![0];
    for (index, cur_design_letter) in design.chars().enumerate() {
        let mut cur = 0;
    }
    //     cur = 0
    //     for j in range(index + 1):
    //         if goal[j:index + 1] in choices and dp[j]:
    //             cur += dp[j]
    //     dp.append(cur)
    // return dp[-1]
}