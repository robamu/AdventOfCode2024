use rayon::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub enum Input {
    Simple,
    Default,
}

const DEBUG_P2: bool = false;

const VISIT_DEBUG: bool = false;

const INPUT: Input = Input::Default;

#[derive(Debug, Default, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub struct Coord2D {
    x: u32,
    y: u32,
}

impl Coord2D {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

pub type PointSet = HashSet<Coord2D>;

#[derive(Debug, Clone)]
pub struct Visited(pub HashMap<Coord2D, HashSet<Direction>>);

#[derive(Debug)]
pub struct PathLoop;

impl Visited {
    pub fn visited_places(&self) -> usize {
        self.0.len()
    }

    pub fn insert(&mut self, coord: Coord2D, dir: Direction) -> Result<(), PathLoop> {
        if VISIT_DEBUG {
            println!(
                "Visited: {:?} (file coord x {} y {})",
                coord,
                coord.x + 1,
                coord.y + 1
            );
        }
        match self.0.entry(coord) {
            std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                if occupied_entry.get().contains(&dir) {
                    return Err(PathLoop);
                }
                occupied_entry.get_mut().insert(dir);
            }
            std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                let mut new_set = HashSet::default();
                new_set.insert(dir);
                vacant_entry.insert(new_set);
            }
        };
        Ok(())
    }
}

#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn turn(self) -> Self {
        // A turn is always a 90 degrees turn clockwise.
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct Guard {
    pub coord: Coord2D,
    pub dir: Direction,
}

impl Guard {
    pub fn update_coord(&mut self, new_coord: Coord2D) {
        self.coord = new_coord;
    }

    pub fn turn(&mut self) {
        self.dir = self.dir.turn();
    }
}

#[derive(Debug, Clone)]
pub struct Lab {
    pub x_dim: u32,
    pub y_dim: u32,
    pub guard_origin: Coord2D,
    pub obstacles: PointSet,
    pub additional_obstacle: Option<Coord2D>,
    visited: Visited,
    pub guard: Guard,
    iterations: u32,
}

impl Lab {
    pub fn new(data: &[u8]) -> Self {
        let mut y_dim = None;
        let mut x_idx = 0;
        let mut y_idx = 0;
        let mut obstacles = HashSet::default();
        let mut guard_coord = None;
        for line in data.lines() {
            let next_line = line.unwrap();
            if !next_line.contains('.') {
                panic!("invalid line detected in input: {}", next_line);
            }
            for character in next_line.chars() {
                if character == '#' {
                    obstacles.insert(Coord2D { x: x_idx, y: y_idx });
                }
                if character == '^' {
                    guard_coord = Some(Coord2D { x: x_idx, y: y_idx });
                }
                y_idx += 1;
            }
            if y_dim.is_none() {
                y_dim = Some(y_idx);
            }
            y_idx = 0;
            x_idx += 1;
        }
        let visited = Visited(HashMap::default());

        Self {
            x_dim: x_idx,
            y_dim: y_dim.unwrap(),
            guard_origin: guard_coord.unwrap(),
            obstacles,
            visited,
            iterations: 0,
            additional_obstacle: None,
            guard: Guard::default(),
        }
    }

    pub fn update_additional_obstacle(&mut self, coord: Coord2D) {
        if let Some(coord) = self.additional_obstacle.take() {
            self.obstacles.remove(&coord);
        }
        self.additional_obstacle = Some(coord);
        self.obstacles.insert(coord);
    }

    pub fn patrol(&mut self) -> Result<Visited, PathLoop> {
        self.visited.0.clear();
        self.guard.coord = self.guard_origin;
        self.guard.dir = Direction::Up;
        self.iterations = 0;
        let mut reached_edge = false;
        while !reached_edge {
            reached_edge = true;
            if self.iterations >= 10000 {
                println!("Additional obstacle: {:?}", self.additional_obstacle);
                panic!("infinite loop detected");
            }
            match self.guard.dir {
                Direction::Up => {
                    for x in (0..=self.guard.coord.x).rev() {
                        if self.obstacles.contains(&Coord2D {
                            x,
                            y: self.guard.coord.y,
                        }) {
                            let new_coord = Coord2D {
                                x: x + 1,
                                y: self.guard.coord.y,
                            };
                            self.guard.update_coord(new_coord);
                            self.guard.turn();
                            reached_edge = false;
                            break;
                        }
                        self.visited.insert(
                            Coord2D {
                                x,
                                y: self.guard.coord.y,
                            },
                            self.guard.dir,
                        )?;
                    }
                    // We reached the upper edge and can walk out of the map.
                }
                Direction::Down => {
                    for x in self.guard.coord.x..self.x_dim {
                        if self.obstacles.contains(&Coord2D {
                            x,
                            y: self.guard.coord.y,
                        }) {
                            let new_coord = Coord2D {
                                x: x - 1,
                                y: self.guard.coord.y,
                            };
                            self.guard.update_coord(new_coord);
                            self.guard.turn();
                            reached_edge = false;
                            break;
                        }
                        self.visited.insert(
                            Coord2D {
                                x,
                                y: self.guard.coord.y,
                            },
                            self.guard.dir,
                        )?;
                    }
                    // We reached the lower edge and can walk out of the map.
                }
                Direction::Left => {
                    for y in (0..=self.guard.coord.y).rev() {
                        if self.obstacles.contains(&Coord2D {
                            x: self.guard.coord.x,
                            y,
                        }) {
                            let new_coord = Coord2D {
                                x: self.guard.coord.x,
                                y: y + 1,
                            };
                            self.guard.update_coord(new_coord);
                            self.guard.turn();
                            reached_edge = false;
                            break;
                        }
                        self.visited.insert(
                            Coord2D {
                                x: self.guard.coord.x,
                                y,
                            },
                            self.guard.dir,
                        )?;
                    }
                    // We reached the left edge and can walk out of the map.
                }
                Direction::Right => {
                    for y in self.guard.coord.y..self.y_dim {
                        if self.obstacles.contains(&Coord2D {
                            x: self.guard.coord.x,
                            y,
                        }) {
                            let new_coord = Coord2D {
                                x: self.guard.coord.x,
                                y: y - 1,
                            };
                            self.guard.update_coord(new_coord);
                            self.guard.turn();
                            reached_edge = false;
                            break;
                        }
                        self.visited.insert(
                            Coord2D {
                                x: self.guard.coord.x,
                                y,
                            },
                            self.guard.dir,
                        )?;
                    }
                    // We reached the right edge and can walk out of the map.
                }
            }
            self.iterations += 1;
        }
        Ok(self.visited.clone())
    }
}

fn main() {
    let filename = match INPUT {
        Input::Simple => "simple.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut lab = Lab::new(&input_file);
    // println!("Lab: {:?}", lab);
    let patrol_result = lab.patrol();
    assert!(patrol_result.is_ok());
    let mut visited = patrol_result.unwrap();
    match INPUT {
        Input::Simple => (),
        Input::Default => {
            assert_eq!(visited.visited_places(), 4826);
        }
    }
    println!(
        "Visited places part 1 for input {:?}: {}",
        INPUT,
        visited.visited_places()
    );
    println!("calculating part 2, takes some time");
    visited.0.remove(&lab.guard_origin);
    // Sort the visited places, makes debugging easier.
    let mut visited_places: Vec<Coord2D> = visited.0.clone().into_keys().collect();
    visited_places.sort_unstable();

    part2_unoptimized(&mut lab, &visited_places);
    part2_parallelized(&mut lab, &visited_places);
}

fn part2_unoptimized(lab: &mut Lab, keys: &[Coord2D]) {
    println!("Part 2 unoptimized");
    let mut loop_obstructions = 0;
    let now = Instant::now();

    // We set an obstacle for each visited path except the start point
    // and then run the patrol algorithm. The loop condition is when
    // the guard re-visits a tile with the same direction.
    for (idx, key) in keys.iter().enumerate() {
        if DEBUG_P2 {
            println!(
                "{}/{}: Re-patrolling with additional obstacle: {:?}",
                idx,
                keys.len(),
                key
            );
        }
        lab.update_additional_obstacle(*key);
        if lab.patrol().is_err() {
            loop_obstructions += 1;
        }
    }
    let elapsed = now.elapsed();
    println!(
        "Possible obstructions for input {:?}: {}, took {} ms",
        INPUT,
        loop_obstructions,
        elapsed.as_millis()
    );
}

// Parallelized with rayon. It's just insane how nice this works.
fn part2_parallelized(lab: &mut Lab, visited: &[Coord2D]) {
    println!("Part 2 parallelized");
    let now = Instant::now();
    let loop_obstructions = AtomicUsize::new(0);
    visited.par_iter().enumerate().for_each(|(_, key)| {
        let mut lab = lab.clone();
        lab.update_additional_obstacle(*key);
        if lab.patrol().is_err() {
            loop_obstructions.fetch_add(1, Ordering::Relaxed);
        }
    });
    let elapsed = now.elapsed();
    println!(
        "Possible obstructions for input {:?}: {}, took {} ms",
        INPUT,
        loop_obstructions.load(Ordering::Relaxed),
        elapsed.as_millis()
    );
}
