#[derive(PartialEq, Debug, Clone, Copy)]
enum Direction {
    Up, Right, Down, Left
}

#[derive(PartialEq, Eq, Clone)]
enum Object {
    Robot,
    Wall,
    Box,
    BoxBigLeft,
    BoxBigRight,
    Empty,
}

fn object_char(obj: &Object) -> char {
    use Object::*;

    match *obj {
        Robot => '@',
        Wall => '#',
        Box => 'O',
        BoxBigLeft => '[',
        BoxBigRight => ']',
        Empty => '.'
    }
}

#[derive(Clone, Copy, Debug)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Vec2 {
    fn from(value: (i32, i32)) -> Self {
        Vec2{
            x: value.0,
            y: value.1
        }
    }
}

impl std::ops::Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}

impl std::ops::Mul<Vec2> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: Vec2) -> Self::Output {
        (self.x * rhs.x, self.y * rhs.y).into()
    }
}

impl std::ops::Sub<Vec2> for Vec2 {
    type Output = Vec2;

    fn sub(self, rhs: Vec2) -> Self::Output {
        (self.x - rhs.x, self.y - rhs.y).into()
    }
}


struct Warehouse {
    board: Grid<Object>,
    robot_pos: Vec2,
    commands: Vec<Direction>,
}

impl Warehouse {
    fn step(&mut self, command: Direction) {
        let direction_vec = match command {
            Direction::Up => (0, -1),
            Direction::Right => (1, 0),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
        };

        let current_pos = self.robot_pos;
        let mut queue = vec![current_pos];
        let mut last_level = vec![current_pos];
        
        let all_empty_finish;
        loop {
            let mut cur_level = vec![];
            for i in 0..last_level.len() {
                let current_pos = last_level.get(i).unwrap().to_owned() + direction_vec.into();
                
                let current_obj =  self.board.get_at(&current_pos);

                if let Some(obj) = current_obj {
                    match obj {
                        Object::BoxBigLeft => {
                            cur_level.push(current_pos);
                            if [Direction::Down, Direction::Up].contains(&command) {
                                cur_level.push(current_pos + (1, 0).into());
                            }
                        }
                        Object::BoxBigRight => {
                            cur_level.push(current_pos);
                            
                            if [Direction::Down, Direction::Up].contains(&command) {
                                cur_level.push(current_pos + (-1, 0).into());
                            }
                        }
                        Object::Wall => {
                            return
                        }
                        Object::Empty => {}
                        _ => {
                            cur_level.push(current_pos);
                        }
                    }

                }
            }

            
            let all_empty = cur_level
                .iter()
                .all(|pos| self.board.get_at(pos).unwrap() == &Object::Empty)
            ;
            
            let is_blocked = cur_level
                .iter()
                .any(|pos| self.board.get_at(pos).unwrap() == &Object::Wall)
            ;
            
            if is_blocked {
                queue = vec![];
            }

            
            queue.extend_from_slice(&cur_level);
            if all_empty {
                all_empty_finish = true;
                break;
            }
            
            last_level = cur_level;
        }

        if !all_empty_finish {
            return
        }
        
        queue.reverse();
        for point in queue.iter() {
            let cur_item = self.board.get_at(point).unwrap().clone();
            if cur_item == Object::Empty {
                continue
            }

            let new_point_coord = *point+direction_vec.into();

            self.board.put_at(new_point_coord, cur_item.clone());
            self.board.put_at(*point, Object::Empty);

            if cur_item == Object::Robot {
                self.robot_pos = new_point_coord;
            }
        }
    }

    fn gps_result(&self) -> i32 {
        self.board
            .map(|(point, val)| {
                match val {
                    Object::Box | Object::BoxBigLeft => point.y * 100 + point.x,
                    _ => 0
                }
            })
            .sum::<i32>()
    }
}

struct Grid<T> {
    data: Vec<T>,
    width: i32,
    height: i32,
}

impl<'this, T> Grid<T> {
    pub fn from_string_with_map<F>(input: &str, map_func: F) -> Self
        where F: Fn(char) -> T {
        
        let height = input.lines().count();
        let width = input.lines().nth(0).unwrap_or("").len();

        let data: Vec<T> = input
            .lines()
            .flat_map(|line| {
                line
                    .chars()
                    .map(&map_func)
                    .collect::<Vec<T>>()
            })
            .collect();

        Grid { data, width: width as i32, height: height as i32 }
    }

    pub fn get_at(&self, p: &Vec2) -> Option<&T> {
        if p.x > self.width || p.y > self.height {
            None
        } else { 
            self.data.get(self.point2pos(p))
        }
    }

    pub fn find<F>(&'this self, f: F) -> impl Iterator<Item = (usize, &'this T)>
        where F: Fn(&'this T) -> bool
    {
        self
            .data
            .iter()
            .enumerate()
            .filter(move |&v| f(v.1))
    }

    pub fn find_one<F>(&'this self, f: F) -> Option<Vec2>
        where F: Fn(&'this T) -> bool
    {
        self
            .find(f)
            .nth(0)
            .map(|(pos, _)| self.pos2point(pos))
    }

    pub fn swap_cells(&mut self, from: &Vec2, to: &Vec2) {
        let from_pos = self.point2pos(from);
        let to_pos = self.point2pos(to);

        self.data.swap(from_pos, to_pos);
    }

    pub fn put_at(&mut self, point: Vec2, item: T) {
        let pos = self.point2pos(&point);
        *self.data.get_mut(pos).unwrap() = item;
    }

    pub fn visualization<F>(&self, map_func: F) -> String 
        where F: Fn(&T) -> char
    {
        self
            .data
            .chunks(self.width as usize)
            .map(|c| {
                c.iter().map(&map_func).collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn is_point_in_boundaries(&self, point: &Vec2) -> bool {
        point.x >= 0 && point.y >= 0
            && point.x < self.width && point.y < self.height
    }

    pub fn map<F, R>(&'this self, f: F) -> impl Iterator<Item = R> + 'this
        where F: Fn((Vec2, &'this T)) -> R + 'this
    {
        self.data
            .iter()
            .enumerate()
            .map(move |(pos, val)| {
                let point = self.pos2point(pos);
                f((point, val))
            })
    }

    fn point2pos(&self, p: &Vec2) -> usize {
        (p.x + p.y * self.width) as usize
    }
    
    fn pos2point(&self, pos: usize) -> Vec2 {
        let y = pos as i32 / self.width;
        let x = pos as i32 % self.width;

        (x, y).into()
    }
}

fn parse_input(input: &str, with_resize: bool) -> Warehouse {
    let input_parts: Vec<&str> = input.split("\n\n").collect();
    let grid_str = {
        let original_grid = input_parts.first().unwrap().to_owned();
        if !with_resize {
            original_grid.to_string()
        } else {
            let mut new_grid = String::with_capacity(original_grid.len()*2);

            original_grid
                .chars()
                .for_each(|c| {
                    match c {
                        '#' => new_grid.push_str("##"),
                        'O' => new_grid.push_str("[]"),
                        '.' => new_grid.push_str(".."),
                        '@' => new_grid.push_str("@."),
                        _ => new_grid.push(c),
                    }
                });

            new_grid
        }
    };

    let commands = input_parts
        .get(1)
        .unwrap()
        .chars()
        .filter(|&c| "^>v<".contains(c))
        .map(|c| {
            match c {
                '^' => Direction::Up,
                '>' => Direction::Right,
                'v' => Direction::Down,
                _ => Direction::Left,
            }
        })
        .collect();

    let grid = Grid::from_string_with_map(grid_str.as_str(), |c| {
        match c {
            '#' => Object::Wall,
            'O' => Object::Box,
            '@' => Object::Robot,
            '[' => Object::BoxBigLeft,
            ']' => Object::BoxBigRight,
            _ => Object::Empty,
        }
    });

    Warehouse{
        robot_pos: grid.find_one(|i| *i == Object::Robot).unwrap(),
        board: grid,
        commands,
    }
}

pub fn process_part1(input: &str) -> String {
    let mut warehouse = parse_input(input, false);

    warehouse
        .commands
        .clone()
        .iter()
        .for_each(|cmd| warehouse.step(*cmd));


    warehouse.gps_result().to_string()
}

pub fn process_part2(input: &str) -> String {
    let mut warehouse = parse_input(input, true);

    warehouse
        .commands
        .clone()
        .iter()
        .for_each(|cmd| warehouse.step(*cmd));
    
    warehouse.gps_result().to_string()
}


pub fn debug_p2(input: &str) -> String {
    let mut warehouse = parse_input(input, false);

    println!("{}", warehouse.board.visualization(object_char));
    warehouse.step(Direction::Up);
    "".to_string()
}


#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_SIMPLE: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const INPUT_SIMPLE2: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn test_process_part1() {
        // println!("{}", process_part1(INPUT_SIMPLE));
        println!("{}", process_part1(INPUT));
    }
    
    #[test]
    fn test_process_part2() {
        // println!("{}", process_part2(INPUT_SIMPLE2));
        println!("{}", process_part2(INPUT));
    }
    const INPUT_DEBUG: &str = "###############
####....[]...##
##....[]....[##
##....[][]...##
##.....[]....##
##[]....@....##
##[][].......##
##..........[##
##..##.......##
##.[][]...[][##
###############

v><>";
    #[test]
    fn test_debug_p2() {
        println!("{}", debug_p2(INPUT_DEBUG));
    }
}


