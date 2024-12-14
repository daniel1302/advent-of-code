#[macro_use]
extern crate bmp;

use std::collections::HashSet;

use nom::{
    bytes::complete::tag, 
    character::complete::{digit1, line_ending}, 
    combinator::{map, opt, recognize},
    sequence::{preceded, separated_pair, terminated, tuple}, 
    IResult
};

type Vec2 = (i32, i32);

#[derive(Debug)]
struct Robot {
    pos: Vec2,
    vel: Vec2,
}

enum Quadrant {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl Robot {
    fn quadrant(&self, board_width: i32, board_height: i32) -> Option<Quadrant> {
        let with_medium = board_width / 2;
        let height_medium = board_height / 2;


        let px = self.pos.0;
        let py  = self.pos.1;


        if px < with_medium && py < height_medium {
            Some(Quadrant::TopLeft)
        } else if px > with_medium && py < height_medium {
            Some(Quadrant::TopRight)
        } else if px < with_medium && py > height_medium {
            Some(Quadrant::BottomLeft)
        } else if px > with_medium && py > height_medium {
            Some(Quadrant::BottomRight)
        } else {
            None
        }
    }

    fn walk(&mut self, board_width: i32, board_height: i32) {
        self.pos = (
            (self.pos.0 + self.vel.0).rem_euclid(board_width),
            (self.pos.1 + self.vel.1).rem_euclid(board_height),
        );
    }
}

fn robot_parser(input: &str) -> IResult<&str, Robot> {  
    terminated(
        map(
            tuple((
                preceded(
                    tag("p="),
                    separated_pair(
                        recognize(preceded(opt(tag("-")), digit1)), 
                        tag(","), 
                        recognize(preceded(opt(tag("-")), digit1))
                    ),
                ),
                preceded(
                    tag(" v="), 
                    separated_pair(
                        recognize(preceded(opt(tag("-")), digit1)), 
                        tag(","), 
                        recognize(preceded(opt(tag("-")), digit1))
                    )),
                )
            ),

            |((px, py), (vx, vy)): ((&str, &str), (&str, &str))| {
                Robot{
                    pos: (
                        px.parse::<i32>().unwrap(),
                        py.parse::<i32>().unwrap(),
                    ),
                    vel: (
                        vx.parse::<i32>().unwrap(),
                        vy.parse::<i32>().unwrap(),
                    ),
                }
            }
        ),
        opt(line_ending)
    )(input)
}


fn parse_input(input: &str) -> Vec<Robot> {
    let mut input = input;
    let mut res = vec![];
    while input.len() > 0 {
        match robot_parser(input) {
            Ok((rest, robot)) => {
                input = rest;
                res.push(robot);
            },
            Err(e) => {
                panic!("{:?}", e);
            },
        }
    }

    res
}

pub fn process_part1(input: &str, board_width: i32, board_height: i32) -> String {
    let mut robots: Vec<Robot> = parse_input(input);
    
    (0..100).for_each(|_| {
        robots
            .iter_mut()
            .for_each(|robot| {
                robot.walk(board_width, board_height);
            });
    });

    
    let quadrants_count = count_robots(&robots, board_width, board_height);
    (quadrants_count.0 * quadrants_count.1 * quadrants_count.2 * quadrants_count.3).to_string()
}

fn count_robots(robots: &Vec<Robot>, board_width: i32, board_height: i32) -> (usize, usize, usize, usize) {
    use Quadrant::*;

    robots
        .iter()
        .fold((0, 0, 0, 0), |mut acc, robot| {
            match robot.quadrant(board_width, board_height) {
                Some(TopLeft) => { acc.0 += 1 },
                Some(TopRight) => { acc.1 += 1 },
                Some(BottomRight) => { acc.2 += 1 },
                Some(BottomLeft) => { acc.3 += 1 },
                None => {}
            }

            acc
        })
}

pub fn process_part2(input: &str, board_width: i32, board_height: i32) -> String {
    let mut robots: Vec<Robot> = parse_input(input);
    
    let mut result = 0;
    (0..7711).for_each(|i| {
        if result > 0 {
            return
        }

        robots
            .iter_mut()
            .for_each(|robot| {
                robot.walk(board_width, board_height);
            });
        if potential_tree(&robots, board_width, board_height) {
            result = i + 1;
            save_robots_bmp(&robots, board_width, board_height, i+1);
        }
    });

    result.to_string()
}

#[allow(unused)]
fn print_robots(robots: &Vec<Robot>, width: i32, height: i32, as_result: bool) -> String {
    let mut board = vec![vec![0; width as usize]; height as usize];

    robots
        .iter()
        .for_each(|r| {
            *board
                .get_mut(r.pos.1 as usize)
                .unwrap()
                    .get_mut(r.pos.0 as usize)
                    .unwrap() += 1;
        });

        board
        .iter()
        .enumerate()
        .map(|(y, line)| line
            .iter()
            .enumerate()
            .map(|(x, n)| {
                if as_result && (y as i32 == height/2 || x as i32 == width/2) {
                    return ' ';
                }
                
                return match *n {
                    0 => '.',
                    r => char::from_digit(r, 10).unwrap()
                } 
            })
            .collect::<String>())
        .collect::<Vec<String>>()
        .join("\n")
}


fn potential_tree(robots: &Vec<Robot>, width: i32, height: i32) -> bool {
    let mut board = vec![vec![0; width as usize]; height as usize];
    let mut robots_num = 0;
    robots
        .iter()
        .for_each(|r| {
            *board
                .get_mut(r.pos.1 as usize)
                .unwrap()
                    .get_mut(r.pos.0 as usize)
                    .unwrap() += 1;

            robots_num += 1;
        });
    
    let mut with_neighbor: HashSet<Vec2> = HashSet::new();

    for x in 1..(width-1) {
        for y in 1..(height-1) {
            let neighbor = [(x, y-1), (x+1, y), (x, y+1), (x-1, y),
                (x-1, y-1), (x+1, y-1), (x-1, y+1), (x+1, y+1)]
                .iter()
                .map(|&(x, y)| {
                    (*board
                        .get(y as usize)
                        .unwrap()
                        .get(x as usize)
                        .unwrap() > 0) as usize
                })
                .sum::<usize>();

            if neighbor > 7 {
                with_neighbor.insert((x, y));
            }
        }
    }

    with_neighbor.len() as f32 >= robots_num as f32 * 0.15
}

fn save_robots_bmp(robots: &Vec<Robot>, width: i32, height: i32, iter: usize) {
    use bmp::{Image, Pixel};
    let mut img = Image::new(width as u32, height as u32);

    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, px!(x, y, 200));
    }

    let mut board = vec![vec![0; width as usize]; height as usize];

    robots
        .iter()
        .for_each(|r| {
            *board
                .get_mut(r.pos.1 as usize)
                .unwrap()
                    .get_mut(r.pos.0 as usize)
                    .unwrap() += 1;
        });

        board
        .iter()
        .enumerate()
        .for_each(|(y, line)| line
            .iter()
            .enumerate()
            .for_each(|(x, n)| {
                if n == &0 {
                    img.set_pixel(x as u32, y as u32, px!(255, 255, 255));
                } else {
                    img.set_pixel(x as u32, y as u32, px!(*n, *n, 200));
                }
            }));

    let _ = img.save(format!("{}.bmp", iter));
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT_SIMPLE: &str = "p=2,4 v=2,-3";
    const INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn test_process_part1() {
        println!("{}", process_part1(INPUT_SIMPLE, 11, 7));
        println!("{}", process_part1(INPUT, 11, 7));
    }
}