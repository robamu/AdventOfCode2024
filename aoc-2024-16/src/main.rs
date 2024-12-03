use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Debug)]
pub enum Input {
    Example0,
    Example1,
    Default,
}

const INPUT: Input = Input::Default;

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

impl Sub for Coord2D {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl Add for Coord2D {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord2D {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        };
    }
}

impl SubAssign for Coord2D {
    fn sub_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        };
    }
}

#[derive(Debug, Hash, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug)]
pub struct Maze {
    pub walls: HashSet<Coord2D>,
    pub start: Coord2D,
    pub end: Coord2D,
    pub x_dim: usize,
    pub y_dim: usize,
}

#[derive(Debug, Copy, Clone)]
pub struct Reindeer {
    pub coord: Coord2D,
    pub score: usize,
    pub direction: Direction,
}

impl Reindeer {
    pub fn r#move(&mut self) {
        match self.direction {
            Direction::North => self.coord -= Coord2D::new(1, 0),
            Direction::South => self.coord += Coord2D::new(1, 0),
            Direction::West => self.coord -= Coord2D::new(0, 1),
            Direction::East => self.coord += Coord2D::new(0, 1),
        }
        self.score += 1;
    }

    pub fn next_coord(&self) -> Coord2D {
        match self.direction {
            Direction::North => self.coord - Coord2D::new(1, 0),
            Direction::South => self.coord + Coord2D::new(1, 0),
            Direction::West => self.coord - Coord2D::new(0, 1),
            Direction::East => self.coord + Coord2D::new(0, 1),
        }
    }

    pub fn turn_right(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::East,
            Direction::South => self.direction = Direction::West,
            Direction::West => self.direction = Direction::North,
            Direction::East => self.direction = Direction::South,
        }
        self.score += 1000;
    }
    pub fn turn_left(&mut self) {
        match self.direction {
            Direction::North => self.direction = Direction::West,
            Direction::South => self.direction = Direction::East,
            Direction::West => self.direction = Direction::South,
            Direction::East => self.direction = Direction::North,
        }
        self.score += 1000;
    }
}

// Global information about the DFS search which also contains the search results.
#[derive(Debug, Default)]
pub struct DfsGlobalInfo {
    pub costs: Vec<usize>,
    pub paths: Vec<HashSet<Coord2D>>,
    pub visited: HashMap<(Coord2D, Direction), usize>,
}

impl Maze {
    pub fn new(data: &[u8]) -> Self {
        let mut walls = HashSet::new();
        let mut start = None;
        let mut end = None;
        let mut y_dim = None;
        let num_lines = data.lines().count();
        let x_dim = num_lines - 2;
        for (x_idx, line) in data.lines().skip(1).enumerate() {
            let line = line.unwrap();
            if x_idx == num_lines {
                continue;
            }
            if y_dim.is_none() {
                y_dim = Some(line.len() - 2);
            }
            let num_chars = line.len();
            for (y_idx, char) in line.chars().skip(1).enumerate() {
                if y_idx == num_chars {
                    continue;
                }
                match char {
                    '#' => {
                        walls.insert(Coord2D::new(x_idx, y_idx));
                    }
                    'S' => start = Some(Coord2D::new(x_idx, y_idx)),
                    'E' => end = Some(Coord2D::new(x_idx, y_idx)),
                    '.' => (),
                    _ => panic!("Invalid character {} in maze", char),
                };
            }
        }
        Self {
            walls,
            start: start.unwrap(),
            end: end.unwrap(),
            x_dim,
            y_dim: y_dim.unwrap(),
        }
    }

    /// DFS search, more or less brute force. Turns are really exensive, so we use a turn
    /// threshold to stop the search and avoid useless paths.
    pub fn cum_costs_dfs_search(&self) -> DfsGlobalInfo {
        let init_reindeer = Reindeer {
            coord: self.start,
            score: 0,
            direction: Direction::East,
        };
        let mut current_path = Vec::new();
        let mut context = DfsGlobalInfo::default();
        self.dfs_pathfinder(init_reindeer, false, &mut current_path, &mut context);
        context
    }

    pub fn find_cheapest_paths(&self) -> (usize, Vec<HashSet<Coord2D>>) {
        let ctx = self.cum_costs_dfs_search();
        let smallest_cost = *ctx.costs.iter().min().unwrap();
        let mut cheapest_paths = Vec::new();
        ctx.costs.iter().enumerate().for_each(|(idx, v)| {
            if *v == smallest_cost {
                cheapest_paths.push(ctx.paths[idx].clone());
            }
        });
        (smallest_cost, cheapest_paths)
    }

    pub fn dfs_pathfinder(
        &self,
        reindeer: Reindeer,
        just_turned: bool,
        current_path: &mut Vec<Coord2D>,
        ctx: &mut DfsGlobalInfo,
    ) {
        // No need to follow a path which is already more expensive than a found one.
        if !ctx.costs.is_empty() && reindeer.score > *ctx.costs.iter().min().unwrap() {
            return;
        }
        let state = (reindeer.coord, reindeer.direction);

        // Check if we've visited this state with a better or equal score
        if let Some(&best_score) = ctx.visited.get(&state) {
            if best_score < reindeer.score {
                return;
            }
        }

        // Record this state with the current score
        ctx.visited.insert(state, reindeer.score);

        // The current path is a stack containing the path taken so far.
        current_path.push(reindeer.coord);

        let handle_next_coord =
            |mut reindeer: Reindeer, current_path: &mut Vec<Coord2D>, ctx: &mut DfsGlobalInfo| {
                let next_coord = reindeer.next_coord();
                if next_coord == self.end {
                    reindeer.r#move();
                    current_path.push(reindeer.coord);
                    ctx.costs.push(reindeer.score);
                    ctx.paths.push(current_path.clone().into_iter().collect());
                    current_path.pop();
                    return;
                }
                if !self.walls.contains(&next_coord) {
                    reindeer.r#move();
                    self.dfs_pathfinder(reindeer, false, current_path, ctx);
                }
            };
        let mut right_turn_was_handled = false;
        let mut left_turn_was_handled = false;
        // Specialized algorithm with heuristics: If we do not go north or east, we try to go there
        // as fast as possible. Also handle north and east direction first, because that's where we
        // need to go.
        match reindeer.direction {
            Direction::North => {
                if reindeer.coord.x > 0 {
                    handle_next_coord(reindeer, current_path, ctx);
                }
            }
            Direction::East => {
                if reindeer.coord.y < self.y_dim - 1 {
                    handle_next_coord(reindeer, current_path, ctx);
                }
            }
            Direction::South => {
                if !just_turned {
                    left_turn_was_handled = true;
                    let mut reindeer_turned_left = reindeer;
                    reindeer_turned_left.turn_left();
                    self.dfs_pathfinder(reindeer_turned_left, true, current_path, ctx);
                }
                if reindeer.coord.x < self.x_dim - 1 {
                    handle_next_coord(reindeer, current_path, ctx);
                }
            }
            Direction::West => {
                if !just_turned {
                    right_turn_was_handled = true;
                    let mut reindeer_turned_right = reindeer;
                    reindeer_turned_right.turn_right();
                    self.dfs_pathfinder(reindeer_turned_right, true, current_path, ctx);
                }
                if reindeer.coord.y > 0 {
                    handle_next_coord(reindeer, current_path, ctx);
                }
            }
        }
        if !just_turned {
            if !right_turn_was_handled {
                let mut reindeer_turned_right = reindeer;
                reindeer_turned_right.turn_right();
                self.dfs_pathfinder(reindeer_turned_right, true, current_path, ctx);
            }
            if !left_turn_was_handled {
                let mut reindeer_turned_left = reindeer;
                reindeer_turned_left.turn_left();
                self.dfs_pathfinder(reindeer_turned_left, true, current_path, ctx);
            }
        }
        current_path.pop();
    }
}

fn main() {
    let start = std::time::Instant::now();
    let filename = match INPUT {
        Input::Example0 => "example0.txt",
        Input::Example1 => "example1.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let maze = Maze::new(&input_file);
    match INPUT {
        Input::Example0 => {
            assert_eq!(maze.start, Coord2D::new(12, 0));
            assert_eq!(maze.end, Coord2D::new(0, 12));
            assert_eq!(maze.x_dim, 13);
            assert_eq!(maze.y_dim, 13);
        }
        Input::Example1 => (),
        Input::Default => (),
    }
    let (cheapest, paths) = maze.find_cheapest_paths();
    println!("Elapsed: {}ms", start.elapsed().as_millis());
    println!("Cheapest path costs: {}", cheapest);
    let mut best_seats = HashSet::new();
    for path in paths {
        for coord in path {
            best_seats.insert(coord);
        }
    }
    println!("Number of best seats: {}", best_seats.len());
    match INPUT {
        Input::Example0 => {
            assert_eq!(cheapest, 7036);
            assert_eq!(best_seats.len(), 45);
        }
        Input::Example1 => {
            assert_eq!(cheapest, 11048);
            assert_eq!(best_seats.len(), 64);
        }
        Input::Default => {
            assert_eq!(cheapest, 89460);
            assert_eq!(best_seats.len(), 504);
        }
    }
}
