use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

const DEBUG: bool = false;

#[derive(Debug)]
pub enum Input {
    Simple,
    Default,
}

const INPUT: Input = Input::Default;

#[derive(Debug, Default, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub struct Coord2D {
    x: i32,
    y: i32,
}

impl Coord2D {
    pub fn has_negative_parts(&self) -> bool {
        self.x < 0 || self.y < 0
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

impl Neg for Coord2D {
    type Output = Self;
    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
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
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

fn main() {
    let filename = match INPUT {
        Input::Simple => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut y_idx = 0;
    let mut x_idx = 0;
    let mut y_dim = 0;
    let mut antennas: HashMap<char, Vec<Coord2D>> = HashMap::default();
    for line in input_file.lines() {
        let next_line = line.unwrap();
        for char in next_line.chars() {
            if char != '.' {
                match antennas.entry(char) {
                    std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(Coord2D::new(x_idx, y_idx));
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![Coord2D::new(x_idx, y_idx)]);
                    }
                }
            }
            y_idx += 1;
        }
        if y_dim == 0 {
            y_dim = y_idx;
        }
        y_idx = 0;
        x_idx += 1;
    }
    let x_dim = x_idx;

    let mut antinodes = HashSet::new();
    // Use brute-force to calculate all unique 2D vectors. Those can be used to find the anti-nodes
    for (char, antenna_with_type) in &antennas {
        if DEBUG {
            println!("Handling antenna type: {:?}", char);
        }
        // The keys are the vectors which were already handled. The value is one of the start point
        // because a vector is not unique on the 2D grid.
        let mut handled_vecs: HashMap<Coord2D, Coord2D> = HashMap::new();
        for (i, coord) in antenna_with_type.iter().enumerate() {
            for (j, other_coord) in antenna_with_type.iter().enumerate() {
                if i == j {
                    continue;
                }
                // We calculate the vector from one point to another.
                let vec = *other_coord - *coord;
                // Also calculate the negation. We want to only handle unique vectors.
                let neg_vec = -vec;
                let vec_already_handled = |v: &Coord2D| {
                    handled_vecs.get(v).map_or(false, |some_start_point| {
                        some_start_point == coord || some_start_point == other_coord
                    })
                };
                // Vector was already handled
                if vec_already_handled(&vec) || vec_already_handled(&neg_vec) {
                    continue;
                }

                if DEBUG {
                    println!("Handling vector from {:?} to {:?}", coord, other_coord);
                    println!("Vector: {:?}", vec);
                }
                handled_vecs.insert(vec, *coord);
                let antinode1 = *other_coord + vec;
                if DEBUG {
                    println!("Detected antinode: {:?}", antinode1);
                }

                let valid_antinode = |antinode: &Coord2D| {
                    !antinode.has_negative_parts() && antinode.x < x_dim && antinode.y < y_dim
                };
                if valid_antinode(&antinode1) {
                    antinodes.insert(antinode1);
                }
                let antinode2 = *coord - vec;
                if DEBUG {
                    println!("Detected antinode: {:?}", antinode2);
                }
                if valid_antinode(&antinode2) {
                    antinodes.insert(antinode2);
                }
            }
        }
    }
    println!("Unique antinodes part 1: {:?}", antinodes.len());
    let mut antinodes = HashSet::new();
    // Use brute-force to calculate all unique 2D vectors. Those can be used to find the anti-nodes
    for (char, antenna_with_type) in &antennas {
        if DEBUG {
            println!("Handling antenna type: {:?}", char);
        }
        // The keys are the vectors which were already handled. The value is one of the start point
        // because a vector is not unique on the 2D grid.
        let mut handled_vecs: HashMap<Coord2D, Coord2D> = HashMap::new();
        for (i, coord) in antenna_with_type.iter().enumerate() {
            antinodes.insert(*coord);
            for (j, other_coord) in antenna_with_type.iter().enumerate() {
                if i == j {
                    continue;
                }
                // We calculate the vector from one point to another.
                let vec = *other_coord - *coord;
                // Also calculate the negation. We want to only handle unique vectors.
                let neg_vec = -vec;
                let vec_already_handled = |v: &Coord2D| {
                    handled_vecs.get(v).map_or(false, |some_start_point| {
                        some_start_point == coord || some_start_point == other_coord
                    })
                };
                // Vector was already handled
                if vec_already_handled(&vec) || vec_already_handled(&neg_vec) {
                    continue;
                }

                if DEBUG {
                    println!("Handling vector from {:?} to {:?}", coord, other_coord);
                    println!("Vector: {:?}", vec);
                }
                handled_vecs.insert(vec, *coord);
                let mut antinode = *other_coord + vec;
                loop {
                    if antinode.has_negative_parts() || antinode.x >= x_dim || antinode.y >= y_dim {
                        break;
                    }
                    if antinodes.insert(antinode) && DEBUG {
                        println!("Detected antinode: {:?}", antinode);
                    }
                    antinode += vec;
                }
                antinode = *coord - vec;
                loop {
                    if antinode.has_negative_parts() || antinode.x >= x_dim || antinode.y >= y_dim {
                        break;
                    }
                    if antinodes.insert(antinode) && DEBUG {
                        println!("Detected antinode: {:?}", antinode);
                    }
                    antinode -= vec;
                }
            }
        }
    }
    println!("Unique antinodes part 2: {:?}", antinodes.len());
}
