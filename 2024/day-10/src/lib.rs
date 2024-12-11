use std::collections::HashSet;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line
                .chars()
                .map(|c| c.to_digit(10).unwrap_or(99) as i32)
                .collect()
        })
        .collect()
}

type Point = (i32, i32);

fn neighbors(point: Point) -> Vec<Point> {
    vec![
        (point.0, point.1-1),
        (point.0+1, point.1),
        (point.0, point.1+1),
        (point.0-1, point.1),
    ]
}

fn find_path(map: &Vec<Vec<i32>>, points: &Vec<Point>) -> Vec<Point> {
    if points.len() < 1{
        return vec![]
    }

    let map_width = map.get(1).unwrap().len();
    let map_height = map.len();

    let next_points: Vec<Point> = points
        .iter()
        .map(|&cur_point| {
            let cur_height = map
                .get(cur_point.1 as usize).unwrap()
                .get(cur_point.0 as usize).unwrap().to_owned();
            neighbors(cur_point)
                .iter()
                .filter(|p| {
                    p.0 >= 0 && p.1 >= 0
                        && p.0 < map_width as i32 && p.1 < map_height as i32
                })
                .filter(|p|{
                    let p_height = map
                    .get(p.1 as usize).unwrap()
                    .get(p.0 as usize).unwrap().to_owned();

                    p_height == cur_height + 1
                })
                .map(|p| *p)
                .collect::<Vec<Point>>()
        })
        .flatten()
        .collect();

    let next_step_res = find_path(map, &next_points);

    next_points
        .iter()
        .filter(|p| { 
            map
                .get(p.1 as usize)
                .unwrap()
                .get(p.0 as usize)
                .unwrap()
                .to_owned() == 9
        })
        .chain(next_step_res.iter())
        .map(|p| *p)
        .collect()
}

fn pos2point(pos: usize, width: usize) -> Point {
    let y = pos / width;
    let x = pos % width;

    (x as i32, y as i32)
}

pub fn process_part1(input: &str) -> String {
    let map = parse_input(input);

    input
        .lines()
        .collect::<String>()
        .match_indices("0")
        .map(|(i, _)| {
            let start_point = pos2point(i, map.get(0).unwrap().len());
            find_path(&map, &vec![start_point])
                .iter()
                .map(|p|*p)
                .collect::<HashSet<Point>>()
                .iter()
                .count()
        })
        .sum::<usize>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
   
    let map = parse_input(input);

    input
        .lines()
        .collect::<String>()
        .match_indices("0")
        .map(|(i, _)| {
            let start_point = pos2point(i, map.get(0).unwrap().len());
            find_path(&map, &vec![start_point])
                .len()
        })
        .sum::<usize>()
        .to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}