mod aoclib;
use crate::aoclib::*;

type Memory = Vec<Vec2>;

#[derive(PartialEq, Eq)]
enum MemoryCell {
    Free,
    Corrupted,
}


fn parse_input(input: &str) -> Memory {
    input
        .lines()
        .filter_map(|l| {
            let elements: Vec<&str> = l.split(",").collect();

            if elements.len() < 2 {
                None
            } else {
                Some((
                    elements.get(0).unwrap().parse::<i32>().unwrap(), 
                    elements.get(1).unwrap().parse::<i32>().unwrap(),
                ).into())
            }
        })
        .collect()

}

pub fn process_part1(input: &str, board_size: i32, memory_bytes: usize) -> String {
    let memory = parse_input(input);

    let graph = Grid::new_empty_with_x_y_map(
        board_size, 
        board_size,
        |x, y| {
            let memory_index = memory
                .iter()
                .position(|&pos| pos == Vec2{x, y});

            match memory_index {
                None => MemoryCell::Free,
                Some(idx) => {
                    if idx < memory_bytes {
                        MemoryCell::Corrupted
                    } else {
                        MemoryCell::Free
                    }
                }
            }
        }
    );

    let distance = graph.dijkstra(
        (0,0).into(),
        graph.size() - (1, 1).into(),
        |_, cell| cell == &MemoryCell::Free,
        |_, _| {1},
    );

    distance.unwrap().to_string()
}

pub fn process_part2(input: &str, board_size: i32) -> String {
    let memory = parse_input(input);

    for i in 404..memory.len() {
        let graph = Grid::new_empty_with_x_y_map(
            board_size, 
            board_size,
            |x, y| {
                let memory_index = memory
                    .iter()
                    .position(|&pos| pos == Vec2{x, y});
    
                match memory_index {
                    None => MemoryCell::Free,
                    Some(idx) => {
                        if idx < i {
                            MemoryCell::Corrupted
                        } else {
                            MemoryCell::Free
                        }
                    }
                }
            }
        );

        // println!("{}", graph.visualization(|cell| {
        //     match cell {
        //         MemoryCell::Free => '.',
        //         MemoryCell::Corrupted => '#',
        //     }
        // }));
    
        println!("{}", i);
        let distance = graph.dijkstra(
            (0,0).into(),
            graph.size() - (1, 1).into(),
            |_, cell| cell == &MemoryCell::Free,
            |_, _| {1},
        );

        if distance.is_none() {
            let res = memory.get(i-1).unwrap();
            return format!("{},{}", res.x, res.y);
        }
    }
    
    "-1".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT, 7, 12));
    }
    
    #[test]
    fn test_process_part2() {
        println!("{}", process_part2(INPUT, 7));
    }
}