mod aoclib;

use itertools::Itertools;
use rayon::prelude::*;

use crate::aoclib::*;

#[derive(PartialEq, Eq)]
enum RacetrackElement {
    Free,
    Start,
    End,
    Wall,
}


pub fn process_part1(input: &str) -> String {
    let graph = Grid::from_string_with_map(
        input,
        |c| {
            match c {
                '#' => RacetrackElement::Wall,
                'S' => RacetrackElement::Start,
                'E' => RacetrackElement::End,
                _ => RacetrackElement::Free,
            }
        }
    );

    let start = graph.find_one(|item| item == &RacetrackElement::Start).unwrap();
    let end = graph.find_one(|item| item == &RacetrackElement::End).unwrap();

    let reference = graph.dijkstra(
        start, 
        end, 
        |_, item| [RacetrackElement::Free, RacetrackElement::End].contains(&item), 
        |_, _| {1},
    ).unwrap();

    (0..graph.raw_size())
        .par_bridge()
        .map(|i| {
            if graph.get_at(&graph.pos2point(i)).unwrap() != &RacetrackElement::Wall {
                return 0
            }
    
            let cur_size = graph.dijkstra(
                start, 
                end, 
                |pos, item| {
                    [RacetrackElement::Free, RacetrackElement::End].contains(&item)
                        || graph.point2pos(pos) == i
                }, 
                |_, _| {1},
            );

            let saved = reference.cost - cur_size.unwrap().cost;
            
            (saved >= 100) as i32
        })
        .sum::<i32>()
        .to_string()
}

pub fn process_part2(input: &str) -> String {
    let graph = Grid::from_string_with_map(
        input,
        |c| {
            match c {
                '#' => RacetrackElement::Wall,
                'S' => RacetrackElement::Start,
                'E' => RacetrackElement::End,
                _ => RacetrackElement::Free,
            }
        }
    );

    let start = graph.find_one(|item| item == &RacetrackElement::Start).unwrap();
    let end = graph.find_one(|item| item == &RacetrackElement::End).unwrap();

    let reference = graph.dijkstra(
        start, 
        end, 
        |_, item| [RacetrackElement::Free, RacetrackElement::End].contains(&item), 
        |_, _| {1},
    ).unwrap();

    let distance = |p1: &Vec2, p2: &Vec2| (p1.x - p2.x).abs() + (p1.y - p2.y).abs();

    reference.path.iter()
        .cartesian_product(reference.path.iter())
        .map(|(s1, s2)| {
            if s1.idx < s2.idx { return 0; }
            let p1 = graph.pos2point(s1.idx);
            let p2 = graph.pos2point(s2.idx);

            let dist = distance(&p1, &p2);
            let saves = (s2.cost - s1.cost).abs()-dist;
            (dist <= 20 && saves >= 100) as i32
        })
        .sum::<i32>()
        .to_string()

}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT));
    }
}