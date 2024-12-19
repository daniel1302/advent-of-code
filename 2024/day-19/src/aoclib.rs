use std::collections::HashSet;

const INFINITY: i32 = i32::MAX;


#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Vec2 {
    pub x: i32,
    pub y: i32,
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

pub struct Grid<T> {
    data: Vec<T>,
    width: i32,
    height: i32,
}

impl<T> Grid<T> {
    const NEIGHBORS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

    pub fn new_from_raw(data: Vec<T>, width: i32) -> Self {
        Grid {
            height: data.len() as i32 / width,
            data,
            width,
        }
    }

    pub fn new_empty_with_x_y_map<F>(width: i32, height: i32, map_func: F) -> Self 
        where F: Fn(i32, i32) -> T {

        let data = (0..height)
            .flat_map(|y| {
                (0..width)
                    .map(|x| {
                        map_func(x, y)
                    })
                    .collect::<Vec<T>>()
            })
            .collect();

        Grid{
            data,
            width,
            height
        }
    }

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

    pub fn find<F>(&self, f: F) -> impl Iterator<Item = (usize, &T)>
        where F: Fn(&T) -> bool
    {
        self
            .data
            .iter()
            .enumerate()
            .filter(move |&v| f(v.1))
    }

    pub fn find_one<F>(& self, f: F) -> Option<Vec2>
        where F: Fn(& T) -> bool
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

    pub fn map<'a, F, R>(&'a self, f: F) -> impl Iterator<Item = R> + 'a
        where F: Fn((Vec2, &T)) -> R + 'a
    {
        self.data
            .iter()
            .enumerate()
            .map(move |(pos, val)| {
                let point = self.pos2point(pos);
                f((point, val))
            })
    }

    pub fn size(&self) -> Vec2 {
        (self.width, self.height).into()
    }

    pub fn dijkstra<F, G>(&self, start: Vec2, finish: Vec2, is_valid_point: F, calculate_distance: G) -> Option<i32>
        where F: Fn(&Vec2, &T) -> bool, // is_valid_point(point: &Vec2, vertex: &T)
              G: Fn(&Vec2, &Vec2) -> i32 // calculate_distance(from: &Vec2, to: &Vec2)
    {
        let mut dist = self.data
            .iter()
            .enumerate()
            .filter_map(|(pos, vertex)| {
                let point = self.pos2point(pos);
                // Remove nodes that are invalid here
                if !is_valid_point(&point, vertex) {
                    return None
                }

                if point == start {
                    Some((point, 0))
                } else {
                    Some((point, INFINITY)) 
                }
            })
            .collect::<Vec<(Vec2, i32)>>();

        let mut nodes_to_visit: HashSet<Vec2> = self.data
            .iter()
            .enumerate()
            .filter_map(|(pos, val)| {
                let point = self.pos2point(pos);
                if !is_valid_point(&point, val) {
                    None
                } else {
                    Some(point)
                }                
            })
            .collect();

        
        // nodes_to_visit.remove(&(0, 0).into());

        while nodes_to_visit.len() > 0 {
            dist.sort_by(|a, b| {
                a.1.partial_cmp(&b.1).unwrap()
            });

            let cur_node = dist
                .iter()
                .filter(|(point, distance) | distance < &INFINITY && nodes_to_visit.contains(point))
                .nth(0)?;            

            if cur_node.0 == finish {
                return Some(cur_node.1);
            }

            // We are already visiting the node
            nodes_to_visit.remove(&cur_node.0);

            // Find neighbors and determine distances
            let neighbors_nodes: Vec<(Vec2, i32)> = Grid::<T>::NEIGHBORS
                .iter()
                .filter_map(|neighbor_vec| {
                    let neighbor_point = cur_node.0 + (*neighbor_vec).into();

                    // already visited?
                    if !nodes_to_visit.contains(&neighbor_point) || !self.is_point_in_boundaries(&neighbor_point) {
                        None
                    } else {
                        let current_distance = cur_node.1;
                        let neighbor_distance = calculate_distance(&cur_node.0, &neighbor_point);

                        let previous_current_distance = dist.iter().find(|(pos, _)| *pos == neighbor_point).unwrap();

                        // println!("\n\nDIST: {:?}", dist);
                        // println!("\n\nnodes_to_visit: {:?}", nodes_to_visit);
                        // println!("FROM {:?}({}) TO {:?}({})", cur_node.0, current_distance,  neighbor_point, neighbor_distance);
                        let full_current_distance = current_distance + neighbor_distance;

                        if previous_current_distance.1 > full_current_distance {
                            Some((neighbor_point, full_current_distance))
                        } else {
                            None
                        }
                    }
                }).collect();
                
                neighbors_nodes.iter().for_each(|&(dest_point, distance)| {
                    let dist_index = dist.iter().position(|(point, _)| point == &dest_point); // if unwrap fails it means the point is invalid

                    match dist_index {
                        None => {},
                        Some(idx) => {
                            *dist.get_mut(idx).unwrap() = (dest_point, distance);
                        }
                    }
                });
        }

        None
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