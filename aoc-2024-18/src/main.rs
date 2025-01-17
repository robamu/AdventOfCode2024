use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub enum Input {
    Example,
    Default,
}

const INPUT: Input = Input::Default;
const DEBUG: bool = false;

#[derive(Debug, Copy, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub struct Coord2D {
    x: usize,
    y: usize,
}

impl Coord2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

impl std::ops::Sub for Coord2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl std::ops::Add for Coord2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl std::ops::AddAssign for Coord2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl std::ops::SubAssign for Coord2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

pub struct Memory {
    pub x_dim: usize,
    pub y_dim: usize,
    pub corrupted: HashSet<Coord2D>,
}

impl Memory {
    pub fn new(x_dim: usize, y_dim: usize, corrupted: HashSet<Coord2D>) -> Self {
        Self {
            x_dim,
            y_dim,
            corrupted,
        }
    }

    pub fn find_shortest_path(&self) -> (Option<usize>, Vec<Vec<Coord2D>>) {
        let coord = Coord2D::new(0, 0);
        let mut visited = HashMap::new();
        let mut path = Vec::new();
        let mut paths = Vec::new();
        self.shortest_path_dfs(coord, &mut visited, &mut path, &mut paths);
        if paths.is_empty() {
            return (None, paths);
        }
        let len = paths.iter().map(|v| v.len()).min().unwrap();
        (Some(len), paths)
    }

    pub fn shortest_path_dfs(
        &self,
        coord: Coord2D,
        visited: &mut HashMap<Coord2D, usize>,
        path: &mut Vec<Coord2D>,
        paths: &mut Vec<Vec<Coord2D>>,
    ) {
        if DEBUG {
            println!("Visiting {:?}", coord);
        }
        path.push(coord);
        let opt_shortest = paths.iter().map(|v| v.len()).min();
        if coord == Coord2D::new(self.x_dim - 1, self.y_dim - 1) {
            if let Some(shortest) = opt_shortest {
                match path.len().cmp(&shortest) {
                    std::cmp::Ordering::Less => {
                        paths.clear();
                        paths.push(path.clone());
                    }
                    std::cmp::Ordering::Equal => {
                        paths.push(path.clone());
                    }
                    _ => (),
                }
            } else {
                paths.push(path.clone());
            }
            path.pop();
            return;
        }
        if let Some(shortest) = opt_shortest {
            if path.len() >= shortest {
                path.pop();
                return;
            }
        }
        match visited.entry(coord) {
            std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                if path.len() >= *occupied_entry.get() {
                    path.pop();
                    return;
                } else {
                    // Update the cost if the current path is shorter
                    occupied_entry.insert(path.len());
                }
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(path.len());
            }
        };
        // Handle south and east first, because that is where we need to go.
        if coord.x < self.x_dim - 1 {
            let east_coord = Coord2D::new(coord.x + 1, coord.y);
            if !self.corrupted.contains(&east_coord) {
                self.shortest_path_dfs(east_coord, visited, path, paths);
            }
        }
        if coord.y < self.y_dim - 1 {
            let south_coord = Coord2D::new(coord.x, coord.y + 1);
            if !self.corrupted.contains(&south_coord) {
                self.shortest_path_dfs(south_coord, visited, path, paths);
            }
        }
        if coord.x > 0 {
            let west_coord = Coord2D::new(coord.x - 1, coord.y);
            if !self.corrupted.contains(&west_coord) {
                self.shortest_path_dfs(west_coord, visited, path, paths);
            }
        }
        if coord.y > 0 {
            let north_coord = Coord2D::new(coord.x, coord.y - 1);
            if !self.corrupted.contains(&north_coord) {
                self.shortest_path_dfs(north_coord, visited, path, paths);
            }
        }

        path.pop();
    }
}

fn main() {
    let start = std::time::Instant::now();
    let filename = match INPUT {
        Input::Example => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut corrupted = Vec::new();
    for line in input_file.lines() {
        let line = line.unwrap();
        let numbers: Vec<usize> = line.split(',').map(|v| v.parse().unwrap()).collect();
        corrupted.push(Coord2D::new(numbers[0], numbers[1]));
    }

    let x_dim;
    let y_dim;
    let mut corruption_idx;
    match INPUT {
        Input::Example => {
            x_dim = 7;
            y_dim = 7;
            corruption_idx = 12;
        }
        Input::Default => {
            x_dim = 71;
            y_dim = 71;
            corruption_idx = 1024;
        }
    };
    let corrupted_set: HashSet<Coord2D> = corrupted[0..corruption_idx].iter().cloned().collect();
    let memory = Memory::new(x_dim, y_dim, corrupted_set);
    let (path_len, shortest_paths) = memory.find_shortest_path();
    let path_len = path_len.unwrap();
    println!("elapsed (p1): {}ms", start.elapsed().as_millis());
    println!("Steps for shortest path: {:?}", path_len - 1);
    if DEBUG {
        println!("Shortest path: {:?}", shortest_paths);
    }
    match INPUT {
        Input::Example => assert_eq!(path_len - 1, 22),
        Input::Default => assert_eq!(path_len - 1, 278),
    }
    corruption_idx += 1;
    loop {
        let corrupted_set: HashSet<Coord2D> =
            corrupted[0..corruption_idx].iter().cloned().collect();
        let next_memory = Memory::new(x_dim, y_dim, corrupted_set);
        let (path_len, _) = next_memory.find_shortest_path();
        if path_len.is_none() {
            println!("elapsed (p2): {}ms", start.elapsed().as_millis());
            println!(
                "corruption index {} with value {:?}",
                corruption_idx, corrupted[corruption_idx - 1]
            );
            break;
        }
        corruption_idx += 1;
    }
}
