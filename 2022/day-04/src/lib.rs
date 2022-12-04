struct Assigment {
    start: u32,
    end: u32,
}

impl Assigment {
    pub fn new(range: &str) -> Self {
        let bounds: Vec<u32> = range.split("-")
            .map(|bound| bound.parse::<u32>().unwrap())
            .collect();
            

        Assigment{
            start: bounds[0],
            end: bounds[1],
        }
    }

    // TODO: should be checked if start < end... but i do not care
    pub fn overlap_full(&self, other: &Self) -> bool {
        self.start <= other.start && self.end >= other.end 
    }

    pub fn overlap_at_all(&self, other: &Self) -> bool {
        self.overlap_full(other) 
            || (self.start >= other.start && self.start <= other.end)
            || (self.end <= other.end && self.end >= other.start)
    }
}

pub fn process_part1(input: &str) -> String {
    input.lines()
        .map(|line| {
            let assigments: Vec<Assigment> = line.split(",")
                .map(|assigment_str| Assigment::new(assigment_str))
                .collect();
            
            assigments[0].overlap_full(&assigments[1]) || assigments[1].overlap_full(&assigments[0])
        })
        .map(|item| item as u32)
        .sum::<u32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    input.lines()
    .map(|line| {
        let assigments: Vec<Assigment> = line.split(",")
            .map(|assigment_str| Assigment::new(assigment_str))
            .collect();
        
        assigments[0].overlap_at_all(&assigments[1]) || assigments[1].overlap_at_all(&assigments[0])
    })
    .map(|item| item as u32)
    .sum::<u32>()
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

    #[test]
    fn part1() {
        assert_eq!(process_part1(INPUT), "2");
    }

    #[test]
    fn part2() {
        assert_eq!(process_part2(INPUT), "4");
    }
}
