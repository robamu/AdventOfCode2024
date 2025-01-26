use std::{
    cell::RefCell,
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub enum Input {
    Example,
    Default,
}

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

#[derive(Debug)]
pub struct Racetrack {
    pub x_dim: usize,
    pub y_dim: usize,
    pub start: Coord2D,
    pub end: Coord2D,
    pub walls: RefCell<HashSet<Coord2D>>,
}

impl Racetrack {
    pub fn new_from_data(data: &[u8]) -> Self {
        let x_dim = data.lines().count() - 2;
        let mut walls = HashSet::new();
        let mut start = Coord2D::default();
        let mut end = Coord2D::default();
        let mut y_dim = 0;
        for (idx, line) in data.lines().enumerate() {
            if idx == 0 || idx == x_dim + 1 {
                continue;
            }
            let line = line.unwrap();
            y_dim = line.chars().count() - 2;
            for (y_idx, char) in line.chars().enumerate() {
                if y_idx == 0 || y_idx == y_dim + 1 {
                    continue;
                }
                if char == '#' {
                    walls.insert(Coord2D::new(idx - 1, y_idx - 1));
                }
                if char == 'S' {
                    start = Coord2D::new(idx - 1, y_idx - 1);
                }
                if char == 'E' {
                    end = Coord2D::new(idx - 1, y_idx - 1);
                }
            }
        }
        Self {
            x_dim,
            y_dim,
            start,
            end,
            walls: RefCell::new(walls),
        }
    }

    pub fn try_all_cheat_combinations(&self) -> HashMap<usize, u32> {
        let walls_snapshot = self.walls.borrow().clone();
        let mut picosecond_with_cheats = HashMap::new();
        for wall in &walls_snapshot {
            if wall.x > 0 && wall.x < self.x_dim - 1 {
                let north = Coord2D::new(wall.x - 1, wall.y);
                let south = Coord2D::new(wall.x + 1, wall.y);
                if !walls_snapshot.contains(&north) && !walls_snapshot.contains(&south) {
                    let (picoseconds, _) = self.find_shortest_path_with_cheat(*wall);
                    *picosecond_with_cheats.entry(picoseconds).or_insert(0_u32) += 1;
                }
            }
            if wall.y > 0 && wall.y < self.y_dim - 1 {
                let west = Coord2D::new(wall.x, wall.y - 1);
                let east = Coord2D::new(wall.x, wall.y + 1);
                if !walls_snapshot.contains(&east) && !walls_snapshot.contains(&west) {
                    let (picoseconds, _) = self.find_shortest_path_with_cheat(*wall);
                    *picosecond_with_cheats.entry(picoseconds).or_insert(0_u32) += 1;
                }
            }
            if DEBUG {
                println!("cheating with wall: {:?}", wall);
            }
        }
        picosecond_with_cheats
    }

    pub fn find_shortest_path_with_cheat(&self, cheats: Coord2D) -> (usize, Vec<Vec<Coord2D>>) {
        let init_walls = self.walls.borrow().clone();
        let coord = self.start;
        let mut visited = HashMap::new();
        let mut path = Vec::new();
        let mut paths = Vec::new();
        self.walls.borrow_mut().remove(&cheats);
        self.shortest_path_dfs(coord, &mut visited, &mut path, &mut paths);
        let len = paths
            .iter()
            .map(|v| v.len())
            .min()
            .map_or(0, |p| p.saturating_sub(1));
        self.walls.replace(init_walls);
        (len, paths)
    }

    pub fn find_shortest_path(&self) -> (usize, Vec<Vec<Coord2D>>) {
        let coord = self.start;
        let mut visited = HashMap::new();
        let mut path = Vec::new();
        let mut paths = Vec::new();
        self.shortest_path_dfs(coord, &mut visited, &mut path, &mut paths);
        let len = paths
            .iter()
            .map(|v| v.len())
            .min()
            .map_or(0, |p| p.saturating_sub(1));
        (len, paths)
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
        if coord == self.end {
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
            if !self.walls.borrow().contains(&east_coord) {
                self.shortest_path_dfs(east_coord, visited, path, paths);
            }
        }
        if coord.y < self.y_dim - 1 {
            let south_coord = Coord2D::new(coord.x, coord.y + 1);
            if !self.walls.borrow().contains(&south_coord) {
                self.shortest_path_dfs(south_coord, visited, path, paths);
            }
        }
        if coord.x > 0 {
            let west_coord = Coord2D::new(coord.x - 1, coord.y);
            if !self.walls.borrow().contains(&west_coord) {
                self.shortest_path_dfs(west_coord, visited, path, paths);
            }
        }
        if coord.y > 0 {
            let north_coord = Coord2D::new(coord.x, coord.y - 1);
            if !self.walls.borrow().contains(&north_coord) {
                self.shortest_path_dfs(north_coord, visited, path, paths);
            }
        }

        path.pop();
    }
}

fn main() {
    let start = std::time::Instant::now();
    let input_file = std::fs::read("input.txt").unwrap();
    let racetrack = Racetrack::new_from_data(&input_file);
    if DEBUG {
        println!("Racetrack: {:?}", racetrack);
    }
    let (picoseconds_no_cheats, _) = racetrack.find_shortest_path();
    let picosecond_with_cheats = racetrack.try_all_cheat_combinations();
    let saved_times: HashMap<usize, u32> = picosecond_with_cheats
        .iter()
        .filter(|(picoseconds, _)| **picoseconds < picoseconds_no_cheats)
        .map(|(picoseconds, times)| (picoseconds_no_cheats - *picoseconds, *times))
        .collect();
    println!("elapsed: {}ms", start.elapsed().as_millis());
    println!("Picoseconds default: {}", picoseconds_no_cheats);
    println!("Saved picoseconds {:?}", saved_times);
    let mut cheats_saving_100ps = 0;
    for (saved_time, num) in saved_times {
        if saved_time >= 100 {
            cheats_saving_100ps += num;
        }
    }
    println!("solution p1: {}", cheats_saving_100ps);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_example() {
        let input_file = std::fs::read("example.txt").unwrap();
        let racetrack = Racetrack::new_from_data(&input_file);
        let (picoseconds_no_cheats, _) = racetrack.find_shortest_path();
        let picosecond_with_cheats = racetrack.try_all_cheat_combinations();
        let saved_times: HashMap<usize, u32> = picosecond_with_cheats
            .iter()
            .filter(|(picoseconds, _)| **picoseconds < picoseconds_no_cheats)
            .map(|(picoseconds, times)| (picoseconds_no_cheats - *picoseconds, *times))
            .collect();
        assert_eq!(racetrack.x_dim, 13);
        assert_eq!(racetrack.y_dim, 13);
        assert_eq!(racetrack.start, Coord2D::new(2, 0));
        assert_eq!(racetrack.end, Coord2D::new(6, 4));
        assert_eq!(*saved_times.get(&64).unwrap(), 1);
        assert_eq!(*saved_times.get(&40).unwrap(), 1);
        assert_eq!(*saved_times.get(&38).unwrap(), 1);
        assert_eq!(*saved_times.get(&20).unwrap(), 1);
        assert_eq!(*saved_times.get(&12).unwrap(), 3);
        assert_eq!(*saved_times.get(&10).unwrap(), 2);
        assert_eq!(*saved_times.get(&8).unwrap(), 4);
        assert_eq!(*saved_times.get(&6).unwrap(), 2);
        assert_eq!(*saved_times.get(&4).unwrap(), 14);
        assert_eq!(*saved_times.get(&2).unwrap(), 14);
    }
}
