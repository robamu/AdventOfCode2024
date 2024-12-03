use std::{collections::HashSet, io::BufRead};

const DEBUG: bool = false;

#[derive(Debug)]
pub enum Input {
    Simple,
    Default,
}

const INPUT: Input = Input::Default;

pub struct Topo {
    pub m: Vec<Vec<u8>>,
    pub x_dim: usize,
    pub y_dim: usize,
    pub trailheads: Vec<(usize, usize)>,
}

pub struct TrailContext {
    pub reached_tops: Option<HashSet<(usize, usize)>>,
    pub score: u32,
}

impl TrailContext {
    pub fn new() -> Self {
        Self {
            reached_tops: Some(HashSet::new()),
            score: 0,
        }
    }

    pub fn new_for_part2() -> Self {
        Self {
            reached_tops: None,
            score: 0,
        }
    }
}

impl Topo {
    pub fn new(data: &[u8]) -> Self {
        let mut trailheads = Vec::new();
        let mut m = Vec::new();
        for (x, line) in data.lines().enumerate() {
            let mut row = Vec::new();
            let line = line.unwrap();
            for (y, char) in line.chars().enumerate() {
                let tile_height = char.to_string().parse::<u8>().unwrap();
                if tile_height == 0 {
                    trailheads.push((x, y));
                }
                row.push(tile_height);
            }
            m.push(row);
        }
        let x_dim = m.len();
        let y_dim = m[0].len();
        Self {
            m,
            x_dim,
            y_dim,
            trailheads,
        }
    }

    pub fn find_trails(&self) {
        let path = Vec::with_capacity(9);
        let mut score_sum_p1 = 0;
        let mut score_sum_p2 = 0;
        for (x, y) in &self.trailheads {
            if DEBUG {
                println!("Checking trailhead at {}, {}", x, y);
            }
            let mut ctx = TrailContext::new();
            self.check_trail(path.clone(), *x, *y, &mut ctx);
            if DEBUG {
                println!("Trail had a score of {} for p1", ctx.score);
            }
            score_sum_p1 += ctx.score;
            let mut ctx = TrailContext::new_for_part2();
            self.check_trail(path.clone(), *x, *y, &mut ctx);
            score_sum_p2 += ctx.score;
            if DEBUG {
                println!("Trail had a score of {} for p2", ctx.score);
            }
        }
        println!("Score sum for topo p1: {}", score_sum_p1);
        println!("Score sum for topo p2: {}", score_sum_p2);
    }

    pub fn check_trail(
        &self,
        mut path: Vec<(usize, usize)>,
        x: usize,
        y: usize,
        ctx: &mut TrailContext,
    ) {
        let cur_height = self.m[x][y];
        // Trail is complete, increase the score of the trailhead.
        if cur_height == 9 {
            if let Some(reached_tops) = &mut ctx.reached_tops {
                if !reached_tops.insert((x, y)) {
                    return;
                }
            }
            path.push((x, y));
            ctx.score += 1;
            if DEBUG {
                println!("completed trail with path {:?}", path);
            }
            return;
        }
        path.push((x, y));
        // Check north
        if x > 0 && self.m[x - 1][y] == cur_height + 1 {
            self.check_trail(path.clone(), x - 1, y, ctx);
        }
        // Check south
        if x < self.x_dim - 1 && self.m[x + 1][y] == cur_height + 1 {
            self.check_trail(path.clone(), x + 1, y, ctx);
        }
        // Check west
        if y > 0 && self.m[x][y - 1] == cur_height + 1 {
            self.check_trail(path.clone(), x, y - 1, ctx);
        }
        // Check east
        if y < self.y_dim - 1 && self.m[x][y + 1] == cur_height + 1 {
            self.check_trail(path, x, y + 1, ctx);
        }
    }
}

fn main() {
    let filename = match INPUT {
        Input::Simple => "example.txt",
        Input::Default => "input.txt",
    };
    let input_file = std::fs::read(filename).unwrap();
    let topo = Topo::new(&input_file);
    topo.find_trails();
}
