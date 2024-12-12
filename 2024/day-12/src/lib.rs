use std::{collections::HashSet, hash::Hash};

type Vec2 = (i32, i32);
type Board = Vec<Vec<char>>;


#[derive(Clone)]
enum Corner {
    TopOuterLeft,
    TopOuterRight,
    BottomOuterRight,
    BottomOuterLeft,
    
    TopInnerLeft,
    TopInnerRight,
    BottomInnerRight,
    BottomInnerLeft,
}

fn neighbors(p: Vec2) -> [Vec2; 4] {
    [
        (p.0, p.1-1), 
        (p.0+1,p.1), 
        (p.0, p.1+1),
        (p.0-1, p.1),
    ]
}

fn get_plant(board: &Board, pos: Vec2) -> Option<char> {
    let board_height = board.len();
    let board_width = board.get(0).unwrap().len();
    
    if pos.0 < 0 || pos.1 < 0 
        || pos.0 >= board_width as i32 || pos.1 >= board_height as i32 {
            return None
        }

    Some(board
        .get(pos.1 as usize)
        .unwrap()
        .get(pos.0 as usize)
        .unwrap()
        .to_owned())
}

fn dfs(board: &Board, visited: &mut HashSet<Vec2>, cur_pos: Vec2, board_size: Vec2) -> HashSet<Vec2> {
    if visited.get(&cur_pos).is_some() {
        return HashSet::new()
    }

    let mut res = HashSet::new();
    res.insert(cur_pos);
    visited.insert(cur_pos);

    for neighbor in neighbors(cur_pos) {
        if get_plant(board, cur_pos) != get_plant(board, neighbor) {
            continue;
        }

        res.extend(dfs(board, visited, neighbor, board_size));
    }

    res
}

fn get_perimeter(plant_group: &HashSet<Vec2>) -> i32 {
    plant_group
        .iter()
        .map(|point| {
            neighbors(*point)
                .iter()
                .map(|neighbor| {
                    match plant_group.get(neighbor) {
                        Some(_) => 0,
                        None => 1,
                    }
                })
                .sum::<i32>()
        })
        .sum()
}

fn parse_input(input: &str) -> Vec<HashSet<Vec2>> {
    let board: Vec<Vec<char>> = input
        .trim()
        .lines()
        .map(|line| {
            line.chars().collect()
        })
        .collect();

    let board_height = board.len();
    let board_width = board.get(0).unwrap().len();

    let mut visited: HashSet<Vec2> = HashSet::new();

    let mut result = vec![];
    for x in 0..board_width {
        for y in 0..board_height {
            if visited.get(&(x as i32, y as i32)).is_some() {
                continue
            }

            result.push(dfs(
                &board, 
                &mut visited, 
                (x as i32, y as i32), 
                (board_width as i32, board_height as i32)
            ));
        }
    }

    result
}

pub fn process_part1(input: &str) -> String {
       let res = parse_input(input)
        .iter()
        .map(|plant_group| {
            plant_group.len() * get_perimeter(&plant_group) as usize
        })
        .sum::<usize>();
   
    res.to_string()
}

pub fn process_part2(input: &str) -> String {
    parse_input(input)
        .iter()
        .map(|plant_group| {
            plant_group.len() * count_sides(&plant_group) as usize
        })
        .sum::<usize>()
        .to_string()
}

fn is_corner(plant_group: &HashSet<Vec2>, cur_point: Vec2, kind: Corner) -> bool {
    let top_neighbor = plant_group.get(&(cur_point.0, cur_point.1-1)).is_some();
    let right_neighbor = plant_group.get(&(cur_point.0+1, cur_point.1)).is_some();
    let bottom_neighbor =plant_group.get(&(cur_point.0, cur_point.1+1)).is_some();
    let left_neighbor = plant_group.get(&(cur_point.0-1, cur_point.1)).is_some();

    let top_right_neighbor = plant_group.get(&(cur_point.0+1, cur_point.1-1)).is_some();
    let top_left_neighbor = plant_group.get(&(cur_point.0-1, cur_point.1-1)).is_some();
    let bottom_right_neighbor = plant_group.get(&(cur_point.0+1, cur_point.1+1)).is_some();
    let bottom_left_neighbor = plant_group.get(&(cur_point.0-1, cur_point.1+1)).is_some();

    match kind {
        Corner::TopOuterLeft => !left_neighbor && !top_neighbor,
        Corner::TopOuterRight => !top_neighbor && !right_neighbor,
        Corner::BottomOuterRight => !bottom_neighbor && !right_neighbor,
        Corner::BottomOuterLeft => !bottom_neighbor && !left_neighbor,

        Corner::TopInnerLeft => top_neighbor && right_neighbor && !top_right_neighbor,
        Corner::TopInnerRight => top_neighbor && left_neighbor && !top_left_neighbor,
        Corner::BottomInnerRight => bottom_neighbor && left_neighbor && !bottom_left_neighbor,
        Corner::BottomInnerLeft => bottom_neighbor && right_neighbor && !bottom_right_neighbor,
    }
}

fn count_sides(plant_group: &HashSet<Vec2>) -> usize {
    plant_group
        .iter()
        .map(|cur_point| {
            [Corner::TopOuterLeft, Corner::TopOuterRight, Corner::BottomOuterRight, Corner::BottomOuterLeft,
                Corner::TopInnerLeft, Corner::TopInnerRight, Corner::BottomInnerRight, Corner::BottomInnerLeft]
                .iter()
                .map(|corner_kind| is_corner(plant_group, *cur_point, corner_kind.clone()))
                .filter(|&is_corner_res| is_corner_res)
                .count()
        })
        .sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_SIMPLE: &str = "AAAA
BBCD
BBCC
EEEC";
    const INPUT: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }

    #[test]
    fn test_process_part2() {
        println!("Simple: {}", process_part2(INPUT_SIMPLE));
        println!("Normal: {}", process_part2(INPUT));
    }
}
