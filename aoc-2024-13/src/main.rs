use regex::Regex;
use std::io::BufRead;

const DEBUG: bool = false;

#[derive(Debug)]
pub enum Input {
    Example,
    Default,
}

const INPUT: Input = Input::Default;

#[derive(Default)]
pub enum ParseState {
    #[default]
    ReadButtonsA,
    ReadButtonsB,
    ReadPrize,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct LinSys2x2 {
    pub col0: (u64, u64),
    pub col1: (u64, u64),
    pub b: (u64, u64),
}

impl LinSys2x2 {
    pub fn determinant(&self) -> i64 {
        (self.col0.0 * self.col1.1) as i64 - (self.col1.0 * self.col0.1) as i64
    }

    pub fn determinant_col0(&self) -> i64 {
        (self.b.0 * self.col1.1) as i64 - (self.col1.0 * self.b.1) as i64
    }

    pub fn determinant_col1(&self) -> i64 {
        (self.b.1 * self.col0.0) as i64 - (self.col0.1 * self.b.0) as i64
    }

    pub fn change_position_p2(&mut self) {
        self.b.0 += 10000000000000;
        self.b.1 += 10000000000000;
    }

    pub fn compute_solution(&self) -> Option<(u64, u64)> {
        let det = self.determinant();
        if det == 0 {
            return None;
        }
        let (mut button_presses, mut button_presses2) = (0, 0);
        if self.determinant() != 0 {
            let det_col0 = self.determinant_col0();
            if det_col0 % det != 0 {
                return None;
            }
            let det_col1 = self.determinant_col1();
            if det_col1 % det != 0 {
                return None;
            }
            button_presses = det_col0 / det;
            button_presses2 = det_col1 / det;
        }
        Some((button_presses as u64, button_presses2 as u64))
    }
}

fn main() {
    let filename = match INPUT {
        Input::Example => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let mut les_list = Vec::new();
    let mut next_les = LinSys2x2::default();
    let mut parse_state = ParseState::default();
    let regex_buttons = Regex::new(r"\w*: X\+(\d+), Y\+(\d+)").unwrap();
    let regex_prize = Regex::new(r"\w*: X\=(\d+), Y\=(\d+)").unwrap();

    for line in input_file.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        match parse_state {
            ParseState::ReadButtonsA => {
                let matches = regex_buttons.captures(&line).unwrap();
                next_les.col0.0 = matches[1].parse().unwrap();
                next_les.col0.1 = matches[2].parse().unwrap();
                parse_state = ParseState::ReadButtonsB;
            }
            ParseState::ReadButtonsB => {
                let matches = regex_buttons.captures(&line).unwrap();
                next_les.col1.0 = matches[1].parse().unwrap();
                next_les.col1.1 = matches[2].parse().unwrap();
                parse_state = ParseState::ReadPrize;
            }
            ParseState::ReadPrize => {
                let matches = regex_prize.captures(&line).unwrap();
                next_les.b.0 = matches[1].parse().unwrap();
                next_les.b.1 = matches[2].parse().unwrap();
                les_list.push(next_les);
                next_les = Default::default();
                parse_state = ParseState::ReadButtonsA;
            }
        }
    }
    let mut tokens = 0;
    for les in &les_list {
        if let Some((a_presses, b_presses)) = les.compute_solution() {
            if a_presses > 100 || b_presses > 100 {
                continue;
            }
            if DEBUG {
                println!("solution: {:?}", (a_presses, b_presses));
            }
            tokens += a_presses * 3 + b_presses;
        }
    }
    println!("Tokens p1: {}", tokens);

    tokens = 0;
    for les in &mut les_list {
        les.change_position_p2();

        if let Some((a_presses, b_presses)) = les.compute_solution() {
            if DEBUG {
                println!("solution: {:?}", (a_presses, b_presses));
            }
            tokens += a_presses * 3 + b_presses;
        }
    }
    println!("Tokens p2: {}", tokens);
}
