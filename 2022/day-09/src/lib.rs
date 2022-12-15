use std::collections::HashSet;

#[derive(Clone)]
enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn compute_tail_position(m: &Move, current_tail_pos: (u32, u32), new_head_pos: (u32, u32)) -> (u32, u32) {
    if new_head_pos == current_tail_pos {
        return current_tail_pos;
    }

    if current_tail_pos.0 != new_head_pos.0 
        && current_tail_pos.1 != new_head_pos.1 
        && i32::abs(new_head_pos.0 as i32 -current_tail_pos.0 as i32) + i32::abs(new_head_pos.1 as i32 -current_tail_pos.1 as i32) == 2 
    {
        return current_tail_pos;
    }

    if i32::abs(new_head_pos.0 as i32 -current_tail_pos.0 as i32) + i32::abs(new_head_pos.1 as i32 -current_tail_pos.1 as i32) == 1 {
        return current_tail_pos;
    }

    return match m {
        Move::Up => (new_head_pos.0, new_head_pos.1+1),
        Move::Down => (new_head_pos.0, new_head_pos.1-1),
        Move::Left => (new_head_pos.0+1, new_head_pos.1),
        Move::Right => (new_head_pos.0-1, new_head_pos.1),
    }
}

pub fn process_part1(input: &str) -> String {
    let mut tail_positions: HashSet<(u32, u32)> = HashSet::new();
    let mut current_head_position = (1000u32, 1000u32);
    let mut current_tail_position = (1000u32, 1000u32);

    input.lines().map(|line| {
        let move_instructions = line.split(" ").collect::<Vec<&str>>();
        return match move_instructions[0] {
            "U" => vec![Move::Up; move_instructions[1].parse::<usize>().unwrap()],
            "D" => vec![Move::Down; move_instructions[1].parse::<usize>().unwrap()],
            "R" => vec![Move::Right; move_instructions[1].parse::<usize>().unwrap()],
            "L" => vec![Move::Left; move_instructions[1].parse::<usize>().unwrap()],
            _ => vec![],
        }
    })
    .flatten()
    .for_each(|m| {
        match m {
            Move::Up => current_head_position.1 += 1,
            Move::Down => current_head_position.1 -= 1,
            Move::Left => current_head_position.0 -= 1,
            Move::Right => current_head_position.0 += 1,
        }
        
        current_tail_position = compute_tail_position(&m, current_tail_position, current_head_position);
        tail_positions.insert(current_tail_position.clone());
    });
    
    tail_positions.len().to_string()
}

fn print(rope: &Vec<(u32, u32)>, width: usize, height: usize) {
    let mut buffer = vec!['.'; width*height];
    
    let calc_pos = |knot: &(u32, u32)| -> usize {
        return (width*knot.1 as usize + knot.0 as usize);
    };

    buffer[calc_pos(&rope[0])] = 'H';

    for (idx, knot_pos) in rope.iter().enumerate().skip(1) {
        buffer[calc_pos(knot_pos)] = (48 + idx as u8) as char;
    }

    println!("{:?}", rope);
    for (pos, ch) in buffer.iter().enumerate() {
        print!("{}", ch);

        if pos % width == 0 {
            println!("");
        }
    }
    println!("\n\n\n");

}


pub fn process_part2(input: &str) -> String {
    let mut tail_positions: HashSet<(u32, u32)> = HashSet::new();

    let mut rope_position: Vec<(u32, u32)> = vec![(15u32, 15u32); 10];
    const HEAD_ID: usize = 0;
    const TAIL_ID: usize = 9;


    input.lines().map(|line| {
        let move_instructions = line.split(" ").collect::<Vec<&str>>();
        return match move_instructions[0] {
            "U" => vec![Move::Up; move_instructions[1].parse::<usize>().unwrap()],
            "D" => vec![Move::Down; move_instructions[1].parse::<usize>().unwrap()],
            "R" => vec![Move::Right; move_instructions[1].parse::<usize>().unwrap()],
            "L" => vec![Move::Left; move_instructions[1].parse::<usize>().unwrap()],
            _ => vec![],
        }
    })
    .flatten()
    .for_each(|m| {
        let mut head_position = rope_position[HEAD_ID];
        match m {
            Move::Up => head_position.1 -= 1,
            Move::Down => head_position.1 += 1,
            Move::Left => head_position.0 -= 1,
            Move::Right => head_position.0 += 1,
        }

        rope_position[HEAD_ID] = head_position;

        for (knot_idx, _knot_position) in rope_position.clone().iter().enumerate().skip(1) {
            rope_position[knot_idx] = compute_tail_position(&m, rope_position[knot_idx], rope_position[knot_idx-1]);
        }
        
        // print(&rope_position, 33, 33);
        println!("{:?}, {}", rope_position[TAIL_ID], tail_positions.len());
        tail_positions.insert(rope_position[TAIL_ID].clone());
    });
    
    tail_positions.len().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    #[ignore]
    fn part1() {
        const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

        assert_eq!(process_part1(INPUT), "13");
    }

    #[test]
    fn part2() {
        const INPUT: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

        
        assert_eq!(process_part2(INPUT), "36");
    }
}
