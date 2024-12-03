use std::{
    collections::HashMap,
    io::BufRead,
    ops::{Add, AddAssign, Sub, SubAssign},
};

const DEBUG: bool = false;

#[derive(Debug)]
pub enum Input {
    Example1,
    Example2,
    Default,
}

const INPUT: Input = Input::Default;

#[derive(Debug, Default, Hash, PartialEq, Eq, Copy, Clone, Ord, PartialOrd)]
pub struct Coord2D {
    x: usize,
    y: usize,
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

#[derive(Debug)]
pub struct GardenUnplotted {
    pub garden: Vec<Vec<char>>,
    pub x_dim: usize,
    pub y_dim: usize,
}

#[derive(Debug)]
pub struct Plot {
    plot_type: char,
    tiles: Vec<Coord2D>,
    fences: Vec<(Coord2D, EdgeDir)>,
    fences_on_one_line: HashMap<(usize, EdgeDir), Vec<usize>>,
}

impl Plot {
    pub fn new(plot_type: char) -> Self {
        Self {
            plot_type,
            tiles: Vec::new(),
            fences: Vec::new(),
            fences_on_one_line: HashMap::new(),
        }
    }

    pub fn area(&self) -> usize {
        self.tiles.len()
    }

    pub fn perimeter(&self) -> usize {
        self.fences.len()
    }

    pub fn cost_p1(&self) -> usize {
        self.area() * self.perimeter()
    }

    pub fn cost_p2(&self) -> usize {
        self.area() * self.sides()
    }

    pub fn sides(&self) -> usize {
        let mut sides = 0;
        for fences_on_one_line in self.fences_on_one_line.values() {
            sides += 1;
            assert!(fences_on_one_line.is_sorted());
            for (seg_first, seg_second) in fences_on_one_line
                .iter()
                .zip(fences_on_one_line.iter().skip(1))
            {
                if seg_second - seg_first > 1 {
                    sides += 1;
                }
            }
        }
        sides
    }
}

#[derive(Debug, Eq, PartialEq, Hash)]
pub enum EdgeDir {
    North,
    South,
    East,
    West,
}

impl GardenUnplotted {
    pub fn new(data: &[u8]) -> Self {
        let mut m = Vec::new();
        let mut plots = HashMap::<char, Vec<Coord2D>>::new();
        for (x, line) in data.lines().enumerate() {
            let mut row = Vec::new();
            let line = line.unwrap();
            for (y, char) in line.chars().enumerate() {
                row.push(char);
                match plots.entry(char) {
                    std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(Coord2D::new(x, y));
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![Coord2D::new(x, y)]);
                    }
                }
            }
            m.push(row);
        }
        let x_dim = m.len();
        let y_dim = m[0].len();

        Self {
            garden: m,
            x_dim,
            y_dim,
        }
    }
}

#[derive(Debug)]
pub struct Garden {
    inner: GardenUnplotted,
    pub plots: HashMap<char, Vec<Plot>>,
}

impl Garden {
    pub fn new(unplotted: GardenUnplotted) -> Self {
        Garden {
            inner: unplotted,
            plots: Default::default(),
        }
    }

    pub fn total_cost_p1(&self) -> usize {
        self.plots
            .values()
            .map(|plots| plots.iter().map(|plot| plot.cost_p1()).sum::<usize>())
            .sum()
    }

    pub fn total_cost_p2(&self) -> usize {
        self.plots
            .values()
            .map(|plots| plots.iter().map(|plot| plot.cost_p2()).sum::<usize>())
            .sum()
    }

    pub fn find_all_plots(&mut self) {
        let mut visited_tiles: HashMap<Coord2D, char> = HashMap::new();
        for (i, row) in self.inner.garden.iter().enumerate() {
            for (j, plot_type) in row.iter().enumerate() {
                if visited_tiles.contains_key(&Coord2D::new(i, j)) {
                    continue;
                }

                let mut plot = Plot::new(*plot_type);
                self.find_plot_recursive(&self.inner.garden, &mut plot, &mut visited_tiles, i, j);
                match self.plots.entry(*plot_type) {
                    std::collections::hash_map::Entry::Occupied(mut occupied_entry) => {
                        occupied_entry.get_mut().push(plot);
                    }
                    std::collections::hash_map::Entry::Vacant(vacant_entry) => {
                        vacant_entry.insert(vec![plot]);
                    }
                }
            }
        }
        self.plots.values_mut().for_each(|plots_list| {
            plots_list.iter_mut().for_each(|plot| {
                plot.fences_on_one_line
                    .values_mut()
                    .for_each(|fences_on_one_line| {
                        fences_on_one_line.sort_unstable();
                    });
            });
        });
    }

    pub fn find_plot_recursive(
        &self,
        garden: &[Vec<char>],
        plot: &mut Plot,
        visited: &mut HashMap<Coord2D, char>,
        x: usize,
        y: usize,
    ) {
        let here = Coord2D::new(x, y);
        if visited.contains_key(&here) {
            return;
        }
        plot.tiles.push(here);
        visited.insert(here, plot.plot_type);

        // North
        if x > 0 && garden[x - 1][y] == plot.plot_type {
            self.find_plot_recursive(garden, plot, visited, x - 1, y);
        }
        if x == 0 || garden[x - 1][y] != plot.plot_type {
            plot.fences.push((here, EdgeDir::North));
            plot.fences_on_one_line
                .entry((x, EdgeDir::North))
                .or_default()
                .push(y);
        }
        // South
        if x < self.inner.x_dim - 1 && garden[x + 1][y] == plot.plot_type {
            self.find_plot_recursive(garden, plot, visited, x + 1, y);
        }
        if x == self.inner.x_dim - 1 || garden[x + 1][y] != plot.plot_type {
            plot.fences.push((here, EdgeDir::South));
            plot.fences_on_one_line
                .entry((x, EdgeDir::South))
                .or_default()
                .push(y);
        }
        // East
        if y < self.inner.y_dim - 1 && garden[x][y + 1] == plot.plot_type {
            self.find_plot_recursive(garden, plot, visited, x, y + 1);
        }
        if y == self.inner.y_dim - 1 || garden[x][y + 1] != plot.plot_type {
            plot.fences.push((here, EdgeDir::East));
            plot.fences_on_one_line
                .entry((y, EdgeDir::East))
                .or_default()
                .push(x);
        }
        // West
        if y > 0 && garden[x][y - 1] == plot.plot_type {
            self.find_plot_recursive(garden, plot, visited, x, y - 1);
        }
        if y == 0 || garden[x][y - 1] != plot.plot_type {
            plot.fences.push((here, EdgeDir::West));
            plot.fences_on_one_line
                .entry((y, EdgeDir::West))
                .or_default()
                .push(x);
        }
    }
}

fn main() {
    let filename = match INPUT {
        Input::Example1 => "example1.txt",
        Input::Example2 => "example2.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let garden = GardenUnplotted::new(&input_file);
    let mut garden = Garden::new(garden);
    garden.find_all_plots();
    if DEBUG {
        for plot_list in &garden.plots {
            for plot in plot_list.1 {
                println!("-- Plot type {} --", plot.plot_type);
                println!("Plot area: {}", plot.area());
                println!("Plot perimeter: {}", plot.perimeter());
                println!("Plot sides: {}", plot.sides());
                println!("Plot cost p1: {}", plot.cost_p1());
                println!("Plot cost p2: {}", plot.cost_p2());
            }
        }
    }
    println!("Garden fencing cost p1: {}", garden.total_cost_p1());
    match INPUT {
        Input::Example1 => assert_eq!(garden.total_cost_p1(), 140),
        Input::Example2 => assert_eq!(garden.total_cost_p1(), 1930),
        Input::Default => assert_eq!(garden.total_cost_p1(), 1494342),
    }
    println!("Garden fencing cost p2: {}", garden.total_cost_p2());
    match INPUT {
        Input::Example1 => assert_eq!(garden.total_cost_p2(), 80),
        Input::Example2 => assert_eq!(garden.total_cost_p2(), 1206),
        Input::Default => assert_eq!(garden.total_cost_p2(), 893676),
    }
}
