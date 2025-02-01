use std::{
    cell::RefCell,
    cmp::min,
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub enum Input {
    Example,
    Default,
}

const DEBUG: bool = false;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

    pub fn find_shortest_path_with_dynamic_programming(
        &self,
        start: Coord2D,
    ) -> HashMap<Coord2D, usize> {
        let mut visited: HashMap<Coord2D, usize> = HashMap::new();

        let mut check_vec = self.check_next_tile(&mut visited, 0, start, None);
        visited.insert(start, 0);

        let mut next_to_check = Vec::new();
        loop {
            for (coord, cost, last_dir) in check_vec.drain(..) {
                next_to_check.extend(self.check_next_tile(
                    &mut visited,
                    cost,
                    coord,
                    Some(last_dir),
                ));
            }
            if next_to_check.is_empty() {
                break;
            }
            std::mem::swap(&mut check_vec, &mut next_to_check);
            next_to_check.clear();
        }
        visited
    }

    /// Use dynamic programming to find the shortest path, but start new checks at specific points
    /// and with a given map containing the visited locations and their costs.
    pub fn find_shortest_path_with_dyn_programming_and_given_init_values(
        &self,
        init_check_points: Vec<(Coord2D, usize)>,
        mut visited: HashMap<Coord2D, usize>,
    ) -> usize {
        let mut check_vec = Vec::new();
        for check_point in init_check_points {
            check_vec.extend(self.check_next_tile(
                &mut visited,
                check_point.1,
                check_point.0,
                None,
            ));
        }

        let mut next_to_check = Vec::with_capacity(self.x_dim * self.y_dim);
        loop {
            for (coord, cost, last_dir) in check_vec.drain(..) {
                next_to_check.extend(self.check_next_tile(
                    &mut visited,
                    cost,
                    coord,
                    Some(last_dir),
                ));
            }
            if next_to_check.is_empty() {
                break;
            }
            std::mem::swap(&mut check_vec, &mut next_to_check);
            next_to_check.clear();
        }
        visited
            .get(&self.end)
            .copied()
            .expect("end tile does not have a cost")
    }
    pub fn handle_cheat_with_complete_recalculation(
        &self,
        coords: (Coord2D, Coord2D),
        wall: Coord2D,
        picosecond_with_cheats: &mut HashMap<usize, u32>,
        base_visited: HashMap<Coord2D, usize>,
    ) {
        // Old implementation
        let picoseconds = self.find_shortest_path_with_cheat(
            wall,
            vec![
                (coords.0, base_visited.get(&coords.0).copied().unwrap()),
                (coords.1, base_visited.get(&coords.1).copied().unwrap()),
            ],
            base_visited.clone(),
        );
        *picosecond_with_cheats.entry(picoseconds).or_insert(0_u32) += 1;
    }

    pub fn handle_cheat_with_relative_cost_calculation(
        &self,
        coords: (Coord2D, Coord2D),
        picoseconds_no_cheats: usize,
        picosecond_with_cheats: &mut HashMap<usize, u32>,
        base_visited: HashMap<Coord2D, usize>,
        cost_map: &mut HashMap<Coord2D, usize>,
    ) {
        let mut handle_init_point = |init_point: Coord2D| -> usize {
            *cost_map.entry(init_point).or_insert_with(|| {
                let visited = self.find_shortest_path_with_dynamic_programming(init_point);
                *visited.get(&self.end).unwrap()
            })
        };
        let cost_to_north = base_visited.get(&coords.0).copied().unwrap();
        let cost_to_south = base_visited.get(&coords.1).copied().unwrap();
        let cost_from_north = handle_init_point(coords.0);
        let cost_from_south = handle_init_point(coords.1);
        let shortest_cheat = min(
            cost_to_north + 2 + cost_from_south,
            cost_to_south + 2 + cost_from_north,
        );
        if shortest_cheat <= picoseconds_no_cheats {
            *picosecond_with_cheats
                .entry(shortest_cheat)
                .or_insert(0_u32) += 1;
        }
    }

    pub fn try_all_cheat_combinations_2ps_cheat(
        &self,
        base_visited: HashMap<Coord2D, usize>,
        cost_map: &mut HashMap<Coord2D, usize>,
    ) -> HashMap<usize, u32> {
        let walls_snapshot = self.walls.borrow().clone();
        let picoseconds_no_cheats = base_visited.get(&self.end).copied().unwrap();
        let mut picosecond_with_cheats = HashMap::new();
        for wall in &walls_snapshot {
            if wall.x > 0 && wall.x < self.x_dim - 1 {
                let north = Coord2D::new(wall.x - 1, wall.y);
                let south = Coord2D::new(wall.x + 1, wall.y);
                if !walls_snapshot.contains(&north) && !walls_snapshot.contains(&south) {
                    self.handle_cheat_with_relative_cost_calculation(
                        (north, south),
                        picoseconds_no_cheats,
                        &mut picosecond_with_cheats,
                        base_visited.clone(),
                        cost_map,
                    );
                }
            }
            if wall.y > 0 && wall.y < self.y_dim - 1 {
                let west = Coord2D::new(wall.x, wall.y - 1);
                let east = Coord2D::new(wall.x, wall.y + 1);
                if !walls_snapshot.contains(&west) && !walls_snapshot.contains(&east) {
                    self.handle_cheat_with_relative_cost_calculation(
                        (west, east),
                        picoseconds_no_cheats,
                        &mut picosecond_with_cheats,
                        base_visited.clone(),
                        cost_map,
                    );
                }
            }
            if DEBUG {
                println!("cheating with wall: {:?}", wall);
            }
        }
        picosecond_with_cheats
    }

    /*
    pub fn try_all_cheat_combinations_20ps_cheat(
        &self,
        base_visited: HashMap<Coord2D, usize>,
    ) -> HashMap<usize, u32> {
        let walls_snapshot = self.walls.borrow().clone();
        let mut picosecond_with_cheats = HashMap::new();
        // TODO: How to even find the cheats? If it is a thin wall, I guess that is the simple
        // case. If it is a thick wall, there might be a lot of combinations.. I guess we have to
        // try all the shortest ones? How to find all of them reliably?

        // TODO: Just brute-force all cheats with 20ps of wallhacks and keep a map or all the
        // the best cheats given the same start and end point. Once we have a complete cost map,
        // the lookups of saving should become fast. For brute-forcing the cheats, we could just
        // use a simpler algorithm like BFS which traverses the walls and checks for exits which
        // are not the start and smaller than 20. We just have to keep the shortest paths from a
        // given start point to all other end points in the range of 20.
        for wall in &walls_snapshot {}
        picosecond_with_cheats
    }
    */

    fn find_shortest_path_with_cheat(
        &self,
        cheats: Coord2D,
        init_check_points: Vec<(Coord2D, usize)>,
        base_visited: HashMap<Coord2D, usize>,
    ) -> usize {
        let init_walls = self.walls.borrow().clone();
        self.walls.borrow_mut().remove(&cheats);
        let len = self.find_shortest_path_with_dyn_programming_and_given_init_values(
            init_check_points,
            base_visited,
        );
        self.walls.replace(init_walls);
        len
    }

    fn check_next_tile(
        &self,
        visited: &mut HashMap<Coord2D, usize>,
        cost: usize,
        coord: Coord2D,
        last_dir: Option<Direction>,
    ) -> Vec<(Coord2D, usize, Direction)> {
        let mut next_coords = Vec::new();
        // Early stop condition if a path to the end was already found. Not fully sure whether it
        // helps that much, but it is not expensive either.
        if let Some(end_cost) = visited.get(&self.end) {
            if cost + 1 > *end_cost {
                return next_coords;
            }
        }
        let mut handle_next_step = |next_coord: Coord2D, dir: Direction| {
            if self.walls.borrow().contains(&next_coord) {
                return;
            }
            match visited.entry(next_coord) {
                std::collections::hash_map::Entry::Occupied(occupied_entry) => {
                    let new_cost_is_cheaper = cost + 1 < *occupied_entry.get();
                    if new_cost_is_cheaper {
                        *occupied_entry.into_mut() = cost + 1;
                        next_coords.push((next_coord, cost + 1, dir));
                    }
                }
                std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                    vacant_entry.insert(cost + 1);
                    next_coords.push((next_coord, cost + 1, dir));
                }
            }
        };

        // Handle south and east first, because that is where we need to go.
        if (last_dir.is_none() || last_dir.unwrap() != Direction::Left) && coord.x < self.x_dim - 1
        {
            let east_coord = Coord2D::new(coord.x + 1, coord.y);
            handle_next_step(east_coord, Direction::Right);
        }
        if (last_dir.is_none() || last_dir.unwrap() != Direction::Up) && coord.y < self.y_dim - 1 {
            let south_coord = Coord2D::new(coord.x, coord.y + 1);
            handle_next_step(south_coord, Direction::Down);
        }
        if (last_dir.is_none() || last_dir.unwrap() != Direction::Right) && coord.x > 0 {
            let west_coord = Coord2D::new(coord.x - 1, coord.y);
            handle_next_step(west_coord, Direction::Left);
        }
        if (last_dir.is_none() || last_dir.unwrap() != Direction::Down) && coord.y > 0 {
            let north_coord = Coord2D::new(coord.x, coord.y - 1);
            handle_next_step(north_coord, Direction::Up);
        }
        next_coords
    }

    pub fn find_shortest_path_with_dfs(&self) -> (usize, Vec<Vec<Coord2D>>) {
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

    fn shortest_path_dfs(
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
    let visited = racetrack.find_shortest_path_with_dynamic_programming(racetrack.start);
    let mut cost_map = HashMap::new();
    let picoseconds_no_cheats = visited.get(&racetrack.end).copied().unwrap();
    cost_map.insert(racetrack.start, picoseconds_no_cheats);
    let picosecond_with_cheats =
        racetrack.try_all_cheat_combinations_2ps_cheat(visited, &mut cost_map);
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
    assert_eq!(cheats_saving_100ps, 1450);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input_file = std::fs::read("example.txt").unwrap();
        let racetrack = Racetrack::new_from_data(&input_file);
        let mut cost_map = HashMap::new();
        let visited = racetrack.find_shortest_path_with_dynamic_programming(racetrack.start);
        let picoseconds_no_cheats = visited.get(&racetrack.end).copied().unwrap();
        cost_map.insert(racetrack.start, picoseconds_no_cheats);
        let picosecond_with_cheats =
            racetrack.try_all_cheat_combinations_2ps_cheat(visited, &mut cost_map);
        let saved_times: HashMap<usize, u32> = picosecond_with_cheats
            .iter()
            .filter(|(picoseconds, _)| **picoseconds < picoseconds_no_cheats)
            .map(|(picoseconds, times)| (picoseconds_no_cheats - *picoseconds, *times))
            .collect();
        assert_eq!(racetrack.x_dim, 13);
        assert_eq!(racetrack.y_dim, 13);
        assert_eq!(racetrack.start, Coord2D::new(2, 0));
        assert_eq!(racetrack.end, Coord2D::new(6, 4));
        println!("saved times: {:?}", saved_times);
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
