use std::io::Write;
use std::{
    collections::{HashMap, HashSet},
    fs::OpenOptions,
    io::BufRead,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use regex::Regex;

#[derive(Debug)]
pub enum Input {
    Example,
    Default,
}

const INPUT: Input = Input::Default;

pub const X_DIM_EXAMPLE: usize = 11;
pub const Y_DIM_EXAMPLE: usize = 7;

pub const X_DIM_INPUT: usize = 101;
pub const Y_DIM_INPUT: usize = 103;

#[derive(Debug, Default, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub struct Coord2D {
    x: usize,
    y: usize,
}

#[derive(Debug, Default, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub struct Velocity {
    x: isize,
    y: isize,
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

impl Coord2D {
    pub fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Robot {
    pub position: Coord2D,
    pub velocity: Velocity,
}

#[derive(Debug, Clone)]
pub struct Bathroom {
    pub robots: Vec<Robot>,
    pub x_dim: usize,
    pub y_dim: usize,
}

impl Bathroom {
    pub fn new_one_robot(robot: Robot, x_dim: usize, y_dim: usize) -> Self {
        Self {
            robots: vec![robot],
            x_dim,
            y_dim,
        }
    }

    pub fn new(data: &[u8], x_dim: usize, y_dim: usize) -> Self {
        let re_robot = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
        let mut robot = Robot::default();
        let mut robots = Vec::new();
        for line in data.lines() {
            let line = line.unwrap();
            let values = re_robot.captures(&line).unwrap();
            robot.position.x = values[1].parse().unwrap();
            robot.position.y = values[2].parse().unwrap();
            robot.velocity.x = values[3].parse().unwrap();
            robot.velocity.y = values[4].parse().unwrap();
            robots.push(robot);
        }
        Self {
            robots,
            x_dim,
            y_dim,
        }
    }

    pub fn safety_factor(&self) -> usize {
        let mut robots_ne = 0;
        let mut robots_nw = 0;
        let mut robots_se = 0;
        let mut robots_sw = 0;
        for robot in self.robots.iter() {
            #[allow(clippy::comparison_chain)]
            if robot.position.x < self.x_dim / 2 {
                if robot.position.y < self.y_dim / 2 {
                    robots_nw += 1;
                } else if robot.position.y > self.y_dim / 2 {
                    robots_sw += 1;
                }
            } else if robot.position.x > self.x_dim / 2 {
                if robot.position.y < self.y_dim / 2 {
                    robots_ne += 1;
                } else if robot.position.y > self.y_dim / 2 {
                    robots_se += 1;
                }
            }
        }
        robots_nw * robots_ne * robots_sw * robots_se
    }

    pub fn step(&mut self) {
        for robot in self.robots.iter_mut() {
            // The remaining euclid calculation effectively corrects a wrap around from left to
            // right by adding the whole x dimension to the negative position to wrap it around
            // to the right side.
            robot.position.x = (robot.position.x as isize + robot.velocity.x)
                .rem_euclid(self.x_dim as isize) as usize;
            robot.position.y = (robot.position.y as isize + robot.velocity.y)
                .rem_euclid(self.y_dim as isize) as usize;
        }
    }
}

#[derive(Default, Debug)]
pub struct BathroomXmasPattern {
    pub x_map: HashMap<usize, Vec<usize>>,
    pub y_map: HashMap<usize, Vec<usize>>,
}

impl BathroomXmasPattern {
    pub fn update(&mut self, robots: &[Robot]) {
        self.x_map.clear();
        self.y_map.clear();
        for robot in robots.iter() {
            self.x_map
                .entry(robot.position.x)
                .or_default()
                .push(robot.position.y);
            self.y_map
                .entry(robot.position.y)
                .or_default()
                .push(robot.position.x);
        }
        for lists in self.x_map.values_mut() {
            lists.sort_unstable();
        }
        for lists in self.y_map.values_mut() {
            lists.sort_unstable();
        }
    }

    pub fn lines_occuring(&self, pos_map: &HashMap<usize, Vec<usize>>) -> u32 {
        let mut line_occured = 0;
        let threshold = 5;
        for lists in pos_map.values() {
            let mut line_found = false;
            let mut prev_diff = 0;
            let mut counter = 0;
            for (next, last) in lists.iter().skip(1).zip(lists.iter()) {
                let next_diff = next - last;
                if next_diff > 0 {
                    if next_diff == prev_diff {
                        counter += 1;
                    } else {
                        counter = 0;
                        prev_diff = next_diff;
                        line_found = false;
                    }
                    if counter >= threshold && !line_found {
                        line_occured += 1;
                        line_found = true;
                    }
                }
            }
        }
        line_occured
    }

    pub fn horiz_lines_occuring(&self) -> u32 {
        self.lines_occuring(&self.x_map)
    }

    pub fn vert_lines_occuring(&self) -> u32 {
        self.lines_occuring(&self.y_map)
    }
}

fn main() {
    let filename = match INPUT {
        Input::Example => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut bathroom = match INPUT {
        Input::Example => Bathroom::new(&input_file, X_DIM_EXAMPLE, Y_DIM_EXAMPLE),
        Input::Default => Bathroom::new(&input_file, X_DIM_INPUT, Y_DIM_INPUT),
    };

    let mut pattern_recognition = BathroomXmasPattern::default();

    if !std::fs::exists("xmas_pat").unwrap() {
        std::fs::create_dir("xmas_pat").unwrap();
    }
    let mut bathroom_p1 = bathroom.clone();
    for _ in 0..100 {
        bathroom_p1.step();
    }
    let safety = bathroom_p1.safety_factor();
    println!("{:?}", safety);
    for time in 1..100000 {
        bathroom.step();
        pattern_recognition.update(&bathroom.robots);
        if pattern_recognition.horiz_lines_occuring() >= 2
            || pattern_recognition.vert_lines_occuring() >= 2
        {
            let mut hash_map = HashSet::new();
            let mut xmas_tree = String::new();
            for robot in bathroom.robots.iter() {
                hash_map.insert((robot.position.x, robot.position.y));
            }
            for y in 0..bathroom.y_dim {
                for x in 0..bathroom.x_dim {
                    if hash_map.contains(&(x, y)) {
                        xmas_tree.push('#');
                    } else {
                        xmas_tree.push('.');
                    }
                }
                xmas_tree.push('\n');
            }
            println!("Detected line patterns at {}", time);
            let mut file = OpenOptions::new()
                .write(true)
                .create(true) // Create the file if it doesn't exist
                .truncate(true) // Truncate the file to 0 bytes
                .open("xmas_pat/tree.txt")
                .unwrap();
            writeln!(&mut file, "{}", time).unwrap();
            file.write_all(xmas_tree.as_bytes()).unwrap();
            assert_eq!(time, 6587);
            break;
        }
    }
}

#[allow(dead_code)]
fn one_robot() {
    let one_robot = Robot {
        position: Coord2D::new(2, 4),
        velocity: Velocity { x: 2, y: -3 },
    };
    let mut bath_with_one_robot = Bathroom::new_one_robot(one_robot, X_DIM_EXAMPLE, Y_DIM_EXAMPLE);
    bath_with_one_robot.step();
    bath_with_one_robot.step();
    println!("{:?}", bath_with_one_robot);
}
