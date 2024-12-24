use std::{cmp::Ordering, collections::{BinaryHeap, HashSet}};

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


#[derive(Debug)]
pub struct DijkstraResult {
    pub cost: i32,
    pub path: Vec<DijkstraState>,
}

#[derive(PartialEq, Eq, Debug)]
pub struct DijkstraState {
    pub cost: i32,
    pub idx: usize,
}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other.cost.cmp(&self.cost)
            .then_with(|| self.idx.cmp(&other.idx))
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
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

    pub fn raw_size(&self) -> usize {
        self.data.len()
    }

    pub fn dijkstra<F, G>(&self, start: Vec2, finish: Vec2, is_valid_point: F, calculate_distance: G) -> Option<DijkstraResult>
        where F: Fn(&Vec2, &T) -> bool, // is_valid_point(point: &Vec2, vertex: &T)
              G: Fn(&Vec2, &Vec2) -> i32, // calculate_distance(from: &Vec2, to: &Vec2)
    {
        let mut queue: BinaryHeap<DijkstraState> = BinaryHeap::new();
        let mut dist: Vec<i32> = vec![INFINITY; self.data.len()];
        let mut previous: Vec<Option<usize>> = vec![None; self.data.len()];

        let start_pos = self.point2pos(&start);
        let finish_pos = self.point2pos(&finish);
        dist[start_pos] = 0;
        queue.push(DijkstraState{idx: start_pos, cost: 0});

        let unwind_path = |previous: &Vec<Option<usize>>, dist: &Vec<i32>, next: usize| {
            let mut next_point = previous[next];
            let mut res: Vec<DijkstraState> = vec![];
            while let Some(next_pos) = next_point {
                res.push(DijkstraState{
                    cost: dist[next_pos],
                    idx: next_pos,
                });
                next_point = previous[next_pos];
            } 
            res.reverse();

            res
        };

        while let Some(DijkstraState{idx, cost}) = queue.pop() {
            if idx == finish_pos {
                return Some(DijkstraResult{
                    cost: cost,
                    path: unwind_path(&previous, &dist, finish_pos)
                }); 
            }

            // better path exists to this point
            if cost > dist[idx] { 
                continue; 
            }

            let current_point = self.pos2point(idx);

            for neighbor_vec in Grid::<T>::NEIGHBORS {
                let neighbor_point = current_point + neighbor_vec.into();

                if !self.is_point_in_boundaries(&neighbor_point) {
                    continue;
                }

                let neighbor_pos = self.point2pos(&neighbor_point);
                if !is_valid_point(&neighbor_point, &self.data[neighbor_pos]) {
                    continue;
                }

                let next = DijkstraState{
                    idx: neighbor_pos, 
                    cost: cost + calculate_distance(&current_point, &neighbor_point),
                };

                if next.cost < dist[next.idx] {
                    // Relaxation, we have now found a better way
                    dist[next.idx] = next.cost;
                    previous[next.idx] = Some(idx);
                    queue.push(next);
                }
            }
        }

        None
    }

    pub fn point2pos(&self, p: &Vec2) -> usize {
        (p.x + p.y * self.width) as usize
    }
    
    pub fn pos2point(&self, pos: usize) -> Vec2 {
        let y = pos as i32 / self.width;
        let x = pos as i32 % self.width;

        (x, y).into()
    }
}


pub struct CircularBuffer<const S: usize, T>
    where T: Copy + Default {
    pub buffer: [T; S],
    idx: usize
}

impl<const S: usize, T: Copy + Default> CircularBuffer<S, T> {
    pub fn new() -> Self {
        CircularBuffer{
            buffer: [T::default(); S],
            idx: 0,
        }
    }

    pub fn push(&mut self, val: T) {
        self.buffer[self.idx%S] = val;
        self.idx += 1;
    }

    pub fn dump(&self) -> [T; S] {
        let mut res = self.buffer;
        res.rotate_right(S-(self.idx%S));
        res
    }

    pub fn is_full(&self) -> bool {
        self.idx>=S
    }
    
    pub fn get_front(&self) -> T {
        self.buffer[self.idx%S]
    }
    
    pub fn get_back(&self) -> T {
        if self.idx == 0 {
            T::default()
        } else {
            self.buffer[(self.idx-1)%S]   
        }
    }
}