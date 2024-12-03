use std::io::BufRead;

fn main() {
    let input_file = std::fs::read("input.txt").unwrap();
    let mut xmas_matrix = XmasMatrix::new(&input_file);
    xmas_matrix.check();
}

pub struct XmasMatrix {
    m: Vec<Vec<char>>,
    num_rows: usize,
    num_cols: usize,
    xmas_count: usize,
    xmas_count_p2: usize,
}

impl XmasMatrix {
    pub fn new(data: &[u8]) -> Self {
        let mut m = Vec::new();
        for line in data.lines() {
            let valid_line = line.unwrap();
            let mut current_row = Vec::new();
            for character in valid_line.chars() {
                current_row.push(character);
            }
            m.push(current_row);
        }
        let num_rows = m.len();
        let num_cols = m[0].len();
        Self {
            m,
            num_rows,
            num_cols,
            xmas_count: 0,
            xmas_count_p2: 0,
        }
    }

    pub fn check(&mut self) {
        for i in 0..self.num_rows {
            for j in 0..self.num_cols {
                self.part1(i, j);
                self.part2(i, j);
            }
        }
        println!("XMAS count: {}", self.xmas_count);
        println!("XMAS count p2: {}", self.xmas_count_p2);
    }

    fn part1(&mut self, i: usize, j: usize) {
        if self.enough_space_in_east_dir_p1(j) {
            if self.east_check_p1(i, j) {
                self.xmas_count += 1;
            }
            if self.enough_space_in_south_dir_p1(i) && self.southeast_check_p1(i, j) {
                self.xmas_count += 1;
            }
            if self.enough_space_in_north_dir_p1(i) && self.northeast_check_p1(i, j) {
                self.xmas_count += 1;
            }
        }
        if self.enough_space_in_west_dir_p1(j) {
            if self.west_check_p1(i, j) {
                self.xmas_count += 1;
            }
            if self.enough_space_in_south_dir_p1(i) && self.southwest_check_p1(i, j) {
                self.xmas_count += 1;
            }
            if self.enough_space_in_north_dir_p1(i) && self.northwest_check_p1(i, j) {
                self.xmas_count += 1;
            }
        }
        if self.enough_space_in_north_dir_p1(i) && self.north_check_p1(i, j) {
            self.xmas_count += 1;
        }
        if self.enough_space_in_south_dir_p1(i) && self.south_check_p1(i, j) {
            self.xmas_count += 1;
        }
    }

    fn enough_space_in_east_dir_p1(&self, j: usize) -> bool {
        j < self.num_cols - 3
    }
    fn enough_space_in_south_dir_p1(&self, i: usize) -> bool {
        i < self.num_rows - 3
    }
    fn enough_space_in_west_dir_p1(&self, j: usize) -> bool {
        j >= 3
    }
    fn enough_space_in_north_dir_p1(&self, i: usize) -> bool {
        i >= 3
    }

    fn east_check_p1(&self, i: usize, j: usize) -> bool {
        self.m[i][j] == 'X'
            && self.m[i][j + 1] == 'M'
            && self.m[i][j + 2] == 'A'
            && self.m[i][j + 3] == 'S'
    }

    fn southeast_check_p1(&self, i: usize, j: usize) -> bool {
        self.m[i][j] == 'X'
            && self.m[i + 1][j + 1] == 'M'
            && self.m[i + 2][j + 2] == 'A'
            && self.m[i + 3][j + 3] == 'S'
    }

    fn south_check_p1(&self, i: usize, j: usize) -> bool {
        self.m[i][j] == 'X'
            && self.m[i + 1][j] == 'M'
            && self.m[i + 2][j] == 'A'
            && self.m[i + 3][j] == 'S'
    }

    fn southwest_check_p1(&self, i: usize, j: usize) -> bool {
        self.m[i][j] == 'X'
            && self.m[i + 1][j - 1] == 'M'
            && self.m[i + 2][j - 2] == 'A'
            && self.m[i + 3][j - 3] == 'S'
    }

    fn west_check_p1(&self, i: usize, j: usize) -> bool {
        self.m[i][j] == 'X'
            && self.m[i][j - 1] == 'M'
            && self.m[i][j - 2] == 'A'
            && self.m[i][j - 3] == 'S'
    }

    fn northwest_check_p1(&self, i: usize, j: usize) -> bool {
        self.m[i][j] == 'X'
            && self.m[i - 1][j - 1] == 'M'
            && self.m[i - 2][j - 2] == 'A'
            && self.m[i - 3][j - 3] == 'S'
    }

    fn north_check_p1(&self, i: usize, j: usize) -> bool {
        self.m[i][j] == 'X'
            && self.m[i - 1][j] == 'M'
            && self.m[i - 2][j] == 'A'
            && self.m[i - 3][j] == 'S'
    }
    fn northeast_check_p1(&self, i: usize, j: usize) -> bool {
        self.m[i][j] == 'X'
            && self.m[i - 1][j + 1] == 'M'
            && self.m[i - 2][j + 2] == 'A'
            && self.m[i - 3][j + 3] == 'S'
    }

    fn enough_surrounding_space(&self, i: usize, j: usize) -> bool {
        j < self.num_cols - 1 && j >= 1 && i >= 1 && i < self.num_rows - 1
    }

    fn part2(&mut self, i: usize, j: usize) {
        if self.m[i][j] == 'A'
            && self.enough_surrounding_space(i, j)
            && ((self.m[i + 1][j + 1] == 'M' && self.m[i - 1][j - 1] == 'S')
                || (self.m[i + 1][j + 1] == 'S' && self.m[i - 1][j - 1] == 'M'))
            && ((self.m[i + 1][j - 1] == 'M' && self.m[i - 1][j + 1] == 'S')
                || (self.m[i + 1][j - 1] == 'S' && self.m[i - 1][j + 1] == 'M'))
        {
            self.xmas_count_p2 += 1;
        }
    }
}
