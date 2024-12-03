use std::{
    collections::{HashMap, HashSet},
    io::BufRead,
};

#[derive(Debug)]
pub enum Input {
    Example0,
    Example1,
    Example2,
    Default,
}

const INPUT: Input = Input::Default;
const DEBUG_P1: bool = false;
const DEBUG_P2: bool = false;

pub type Coord = (usize, usize);

#[derive(Debug, Clone)]
pub struct WarehouseCommon {
    pub x_dim: usize,
    pub y_dim: usize,
    pub robot: Coord,
    pub walls: HashSet<Coord>,
}

impl WarehouseCommon {
    pub fn next_move_into_border(&self, dir: Direction, coord: Coord) -> bool {
        match dir {
            Direction::Up => {
                if coord.0 == 0 {
                    return true;
                }
            }
            Direction::Down => {
                if coord.0 >= self.x_dim - 1 {
                    return true;
                }
            }
            Direction::Left => {
                if coord.1 == 0 {
                    return true;
                }
            }
            Direction::Right => {
                if coord.1 >= self.y_dim - 1 {
                    return true;
                }
            }
        }
        false
    }

    pub fn apply_movement(movement: Direction, mut coords: (usize, usize)) -> (usize, usize) {
        match movement {
            Direction::Up => coords.0 -= 1,
            Direction::Down => coords.0 += 1,
            Direction::Left => coords.1 -= 1,
            Direction::Right => coords.1 += 1,
        }
        coords
    }

    pub fn parse_movement_line(char_vec: &[char], movements: &mut Vec<Direction>) {
        for c in char_vec {
            match c {
                '>' => movements.push(Direction::Right),
                '<' => movements.push(Direction::Left),
                '^' => movements.push(Direction::Up),
                'v' => movements.push(Direction::Down),
                _ => panic!("Invalid character in movement: {}", c),
            }
        }
    }
}

#[derive(Default, Debug, Copy, Clone)]
pub enum ParseState {
    #[default]
    Warehouse,
    Movement,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone)]
pub struct Warehouse {
    pub inner: WarehouseCommon,
    pub crates: HashSet<Coord>,
}

impl Warehouse {
    pub fn new(data: &[u8], movements: &mut Vec<Direction>) -> Self {
        let mut parse_state = ParseState::default();
        let mut x_coord = 0;
        let mut walls = HashSet::new();
        let mut crates = HashSet::new();
        let mut robot = (0, 0);
        let mut y_dim = 0;
        for line in data.lines() {
            let line = line.unwrap();
            let char_vec = line.chars().collect::<Vec<char>>();
            match parse_state {
                ParseState::Warehouse => {
                    if char_vec.is_empty() {
                        parse_state = ParseState::Movement;
                        continue;
                    }
                    y_dim = char_vec.len() - 2;
                    if char_vec.contains(&'O') || char_vec.contains(&'.') {
                        for (idx, c) in char_vec.iter().skip(1).enumerate() {
                            if idx == y_dim {
                                break;
                            }
                            if *c == '#' {
                                walls.insert((x_coord, idx));
                            } else if *c == 'O' {
                                crates.insert((x_coord, idx));
                            } else if *c == '@' {
                                robot = (x_coord, idx);
                            }
                        }
                        x_coord += 1;
                    }
                }
                ParseState::Movement => {
                    WarehouseCommon::parse_movement_line(&char_vec, movements);
                }
            }
        }
        Self {
            inner: WarehouseCommon {
                x_dim: x_coord,
                y_dim,
                walls,
                robot,
            },
            crates,
        }
    }

    pub fn apply_movement_wide_crate(movement: Direction, mut wide_crate: WideCrate) -> WideCrate {
        match movement {
            Direction::Up => {
                wide_crate.left.0 -= 1;
                wide_crate.right.0 -= 1;
            }
            Direction::Down => {
                wide_crate.left.0 += 1;
                wide_crate.right.0 += 1;
            }
            Direction::Left => {
                wide_crate.left.1 -= 1;
                wide_crate.right.1 -= 1;
            }
            Direction::Right => {
                wide_crate.left.1 += 1;
                wide_crate.right.1 += 1;
            }
        }
        wide_crate
    }

    pub fn sum_of_gps(&self) -> usize {
        let mut sum = 0;
        for wcrate in self.crates.iter() {
            sum += (wcrate.0 + 1) * 100 + wcrate.1 + 1;
        }
        sum
    }

    pub fn move_robot(&mut self, movements: &[Direction]) {
        for &movement in movements {
            if self.inner.next_move_into_border(movement, self.inner.robot) {
                continue;
            }
            let next_coord = WarehouseCommon::apply_movement(movement, self.inner.robot);
            if self.inner.walls.contains(&next_coord) {
                continue;
            }
            let mut move_robot = true;
            if self.crates.contains(&next_coord) {
                let mut move_coord = next_coord;
                loop {
                    if self.inner.next_move_into_border(movement, move_coord) {
                        move_robot = false;
                        break;
                    }
                    move_coord = WarehouseCommon::apply_movement(movement, move_coord);
                    if self.crates.contains(&move_coord) {
                        continue;
                    }
                    if self.inner.walls.contains(&move_coord) {
                        move_robot = false;
                        break;
                    }
                    self.crates.remove(&next_coord);
                    self.crates.insert(move_coord);
                    break;
                }
            }
            if move_robot {
                self.inner.robot = next_coord;
            }
            if DEBUG_P1 {
                println!("robot: {:?}", self.inner.robot);
                println!("crates: {:?}", self.crates);
            }
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct WideCrate {
    left: Coord,
    right: Coord,
}

#[derive(Debug, Clone)]
pub struct WideWarehouse {
    pub inner: WarehouseCommon,
    pub crate_locations: HashMap<Coord, usize>,
    pub wide_crates: Vec<WideCrate>,
}

impl WideWarehouse {
    pub fn new(
        robot: Coord,
        x_dim: usize,
        y_dim: usize,
        walls: HashSet<Coord>,
        wide_crates: HashSet<WideCrate>,
    ) -> Self {
        let mut wide_crate_list = Vec::new();
        let mut crate_locations = HashMap::new();
        for (idx, wide_crate) in wide_crates.iter().enumerate() {
            wide_crate_list.push(*wide_crate);
            crate_locations.insert(wide_crate.left, idx);
            crate_locations.insert(wide_crate.right, idx);
        }
        Self {
            inner: WarehouseCommon {
                x_dim,
                y_dim,
                robot,
                walls,
            },
            crate_locations,
            wide_crates: wide_crate_list,
        }
    }
    pub fn new_from_data(data: &[u8], movements: &mut Vec<Direction>) -> Self {
        let mut parse_state = ParseState::default();
        let mut x_coord = 0;
        let mut walls = HashSet::new();
        let mut wide_crates = Vec::new();
        let mut crate_locations = HashMap::new();
        let mut current_crate_idx = 0;
        let mut robot = (0, 0);
        let mut y_dim = 0;
        for line in data.lines() {
            let line = line.unwrap();
            let char_vec = line.chars().collect::<Vec<char>>();
            match parse_state {
                ParseState::Warehouse => {
                    if char_vec.is_empty() {
                        parse_state = ParseState::Movement;
                        continue;
                    }
                    y_dim = char_vec.len() * 2 - 4;
                    if char_vec.contains(&'O') || char_vec.contains(&'.') {
                        for (idx, c) in char_vec.iter().skip(1).enumerate() {
                            let y_coord = idx * 2;
                            if y_coord == y_dim {
                                break;
                            }
                            if *c == '#' {
                                walls.insert((x_coord, y_coord));
                                walls.insert((x_coord, y_coord + 1));
                            } else if *c == 'O' {
                                crate_locations.insert((x_coord, y_coord), current_crate_idx);
                                crate_locations.insert((x_coord, y_coord + 1), current_crate_idx);
                                wide_crates.push(WideCrate {
                                    left: (x_coord, y_coord),
                                    right: (x_coord, y_coord + 1),
                                });
                                current_crate_idx += 1;
                            } else if *c == '@' {
                                robot = (x_coord, y_coord);
                            }
                        }
                        x_coord += 1;
                    }
                }
                ParseState::Movement => {
                    WarehouseCommon::parse_movement_line(&char_vec, movements);
                }
            }
        }
        Self {
            inner: WarehouseCommon {
                x_dim: x_coord,
                y_dim,
                walls,
                robot,
            },
            crate_locations,
            wide_crates,
        }
    }

    pub fn print_warehouse(&self) {
        for x in 0..self.inner.x_dim {
            for y in 0..self.inner.y_dim {
                let coord = (x, y);
                if self.inner.walls.contains(&coord) {
                    print!("#");
                } else if self.crate_locations.contains_key(&coord) {
                    let wcrate_idx = self.crate_locations[&coord];
                    if self.wide_crates[wcrate_idx].left == coord {
                        print!("[");
                    } else {
                        print!("]");
                    }
                } else if self.inner.robot == coord {
                    print!("@");
                } else {
                    print!(".");
                }
            }
            println!();
        }
    }
    pub fn handle_crate_up_down_dir_llm(
        &self,
        movement: Direction,
        next_coord: Coord,
        crates_to_move: &mut Vec<usize>,
    ) -> bool {
        let mut move_robot = true;
        let mut crates_to_check: HashSet<usize> =
            HashSet::from([self.crate_locations[&next_coord]]);
        let mut next_crates_to_check = HashSet::new();

        while move_robot && !crates_to_check.is_empty() {
            for &wcrate_idx in &crates_to_check {
                let wide_crate = self.wide_crates[wcrate_idx];

                // Determine next coordinates based on the direction.
                let (next_coord_left, next_coord_right) = match movement {
                    Direction::Up => (
                        (wide_crate.left.0.wrapping_sub(1), wide_crate.left.1),
                        (wide_crate.right.0.wrapping_sub(1), wide_crate.right.1),
                    ),
                    Direction::Down => (
                        (wide_crate.left.0 + 1, wide_crate.left.1),
                        (wide_crate.right.0 + 1, wide_crate.right.1),
                    ),
                    _ => unreachable!(),
                };

                // Check for boundaries and walls.
                if next_coord_left.0 >= self.inner.x_dim
                    || next_coord_right.0 >= self.inner.x_dim
                    || self.inner.walls.contains(&next_coord_left)
                    || self.inner.walls.contains(&next_coord_right)
                {
                    move_robot = false;
                    break;
                }

                // Add the current crate to the list of crates to move.
                crates_to_move.push(wcrate_idx);

                // Check for crates in the next positions.
                for &coord in &[next_coord_left, next_coord_right] {
                    if let Some(&next_crate_idx) = self.crate_locations.get(&coord) {
                        next_crates_to_check.insert(next_crate_idx);
                    }
                }
            }

            // Swap and clear sets for the next iteration.
            std::mem::swap(&mut crates_to_check, &mut next_crates_to_check);
            next_crates_to_check.clear();
        }

        move_robot
    }

    pub fn handle_crate_up_down_dir(
        &self,
        movement: Direction,
        next_coord: Coord,
        crates_to_move: &mut Vec<usize>,
    ) -> bool {
        let mut move_robot = true;

        let mut crates_to_check: HashSet<usize> =
            HashSet::from([self.crate_locations[&next_coord]]);
        let mut next_crates_to_check = HashSet::new();
        while move_robot && !crates_to_check.is_empty() {
            for &wcrate_idx in &crates_to_check {
                let wide_crate = self.wide_crates[wcrate_idx];
                let next_coord_left;
                let next_coord_right;
                if movement == Direction::Up {
                    if wide_crate.left.0 == 0 {
                        move_robot = false;
                        break;
                    }
                    next_coord_left = (wide_crate.left.0 - 1, wide_crate.left.1);
                    next_coord_right = (wide_crate.right.0 - 1, wide_crate.right.1);
                } else {
                    if wide_crate.left.0 == self.inner.x_dim - 1 {
                        move_robot = false;
                        break;
                    }
                    next_coord_left = (wide_crate.left.0 + 1, wide_crate.left.1);
                    next_coord_right = (wide_crate.right.0 + 1, wide_crate.right.1);
                }
                if self.inner.walls.contains(&next_coord_left)
                    || self.inner.walls.contains(&next_coord_right)
                {
                    move_robot = false;
                    break;
                }
                crates_to_move.push(wcrate_idx);

                // Check for crates in the next positions.
                for &coord in &[next_coord_left, next_coord_right] {
                    if let Some(&next_crate_idx) = self.crate_locations.get(&coord) {
                        next_crates_to_check.insert(next_crate_idx);
                    }
                }
            }
            std::mem::swap(&mut crates_to_check, &mut next_crates_to_check);
            next_crates_to_check.clear();
        }
        move_robot
    }

    pub fn handle_crate_left_right_dir(
        &self,
        movement: Direction,
        next_coord: Coord,
        crates_to_move: &mut Vec<usize>,
    ) -> bool {
        let mut move_robot = true;
        // Similar to the default logic of part 1, but the crates are wider.
        let mut current_coord = next_coord;
        crates_to_move.push(self.crate_locations[&current_coord]);
        loop {
            if movement == Direction::Left {
                if current_coord.1 == 1 {
                    move_robot = false;
                    break;
                }
                current_coord.1 = current_coord.1.saturating_sub(2);
            } else {
                if current_coord.1 == self.inner.y_dim - 2 {
                    move_robot = false;
                    break;
                }
                current_coord.1 += 2;
            }
            if self.inner.walls.contains(&current_coord) {
                move_robot = false;
                break;
            }
            // There are no further crates to push.
            if !self.crate_locations.contains_key(&current_coord) {
                break;
            }
            // There is another crate we can push.
            crates_to_move.push(self.crate_locations[&current_coord]);
        }
        move_robot
    }

    pub fn move_robot(&mut self, movements: &[Direction]) {
        for &movement in movements {
            if DEBUG_P2 {
                println!("next movement: {:?}", movement);
            }
            if self.inner.next_move_into_border(movement, self.inner.robot) {
                continue;
            }
            let next_coord = WarehouseCommon::apply_movement(movement, self.inner.robot);
            if self.inner.walls.contains(&next_coord) {
                continue;
            }
            let mut move_robot = true;
            if self.crate_locations.contains_key(&next_coord) {
                let mut crates_to_move = Vec::new();
                move_robot = match movement {
                    Direction::Up | Direction::Down => {
                        self.handle_crate_up_down_dir(movement, next_coord, &mut crates_to_move)
                    }
                    Direction::Left | Direction::Right => {
                        // Similar to the default logic of part 1, but the crates are wider.
                        self.handle_crate_left_right_dir(movement, next_coord, &mut crates_to_move)
                    }
                };
                if move_robot {
                    if DEBUG_P2 && !crates_to_move.is_empty() {
                        println!("crates to move into {:?}: {:?}", movement, crates_to_move);
                    }
                    if !crates_to_move.is_empty() {
                        self.translate_crates(movement, &crates_to_move);
                    }
                }
            }
            if move_robot {
                self.inner.robot = next_coord;
            }
            if DEBUG_P2 {
                self.print_warehouse();
            }
        }
    }

    pub fn sum_of_gps(&self) -> usize {
        let mut sum = 0;
        for wcrate in self.wide_crates.iter() {
            sum += (wcrate.left.0 + 1) * 100 + wcrate.left.1 + 2;
        }
        sum
    }

    pub fn translate_crates(&mut self, dir: Direction, crate_indexes: &[usize]) {
        crate_indexes.iter().for_each(|&idx| {
            self.crate_locations
                .remove(&self.wide_crates[idx].left)
                .unwrap();
            self.crate_locations
                .remove(&self.wide_crates[idx].right)
                .unwrap();
        });

        crate_indexes.iter().for_each(|&idx| match dir {
            Direction::Up => {
                self.wide_crates[idx].left.0 -= 1;
                self.wide_crates[idx].right.0 -= 1;
            }
            Direction::Down => {
                self.wide_crates[idx].left.0 += 1;
                self.wide_crates[idx].right.0 += 1;
            }
            Direction::Left => {
                self.wide_crates[idx].left.1 -= 1;
                self.wide_crates[idx].right.1 -= 1;
            }
            Direction::Right => {
                self.wide_crates[idx].left.1 += 1;
                self.wide_crates[idx].right.1 += 1;
            }
        });

        crate_indexes.iter().for_each(|&idx| {
            assert!(self
                .crate_locations
                .insert(self.wide_crates[idx].left, idx)
                .is_none());
            assert!(self
                .crate_locations
                .insert(self.wide_crates[idx].right, idx)
                .is_none());
        });
    }
}

fn main() {
    let filename = match INPUT {
        Input::Example0 => "example0.txt",
        Input::Example1 => "example1.txt",
        Input::Example2 => "example2.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut movements = Vec::new();
    let mut warehouse = Warehouse::new(&input_file, &mut movements);
    warehouse.move_robot(&movements);
    let sum_of_gps = warehouse.sum_of_gps();
    println!("sum of gps: {}", sum_of_gps);
    match INPUT {
        Input::Example0 => assert_eq!(sum_of_gps, 2028),
        Input::Example1 => assert_eq!(sum_of_gps, 10092),
        Input::Example2 => (),
        Input::Default => assert_eq!(sum_of_gps, 1294459),
    }

    let mut movements = Vec::new();
    let mut wide_warehouse = WideWarehouse::new_from_data(&input_file, &mut movements);
    if DEBUG_P2 {
        wide_warehouse.print_warehouse();
    }
    wide_warehouse.move_robot(&movements);
    let sum_of_gps = wide_warehouse.sum_of_gps();
    println!("sum of gps (wide): {}", sum_of_gps);
    match INPUT {
        Input::Example0 => (),
        Input::Example1 => assert_eq!(sum_of_gps, 9021),
        Input::Example2 => assert_eq!(sum_of_gps, 618),
        Input::Default => assert_eq!(sum_of_gps, 1319212),
    }
}
